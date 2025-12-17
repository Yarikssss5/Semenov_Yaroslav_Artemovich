#!/bin/bash

# Мониторинг доступности сайтов
# Автор: SysAdmin
# Версия: 2.0

# ==========================================
# КОНФИГУРАЦИЯ
# ==========================================
CONFIG_FILE="monitoring.conf"     # Файл конфигурации
URL_FILE="urls.txt"               # Файл с URL для мониторинга
LOG_FILE="monitoring.log"         # Основной лог-файл
ERROR_LOG="errors.log"            # Лог ошибок
STATUS_FILE="status.json"         # Файл статусов в JSON
CHECK_INTERVAL=5                  # Интервал проверки в минутах (по умолчанию)
TIMEOUT=10                        # Таймаут запроса в секундах
MAX_RETRIES=3                     # Количество попыток при ошибке
SEND_EMAIL_ON_ERROR=false         # Отправлять email при ошибке
EMAIL_ADDRESS="admin@example.com" # Email для уведомлений

# ==========================================
# ФУНКЦИИ
# ==========================================

# Функция загрузки конфигурации
load_config() {
    if [ -f "$CONFIG_FILE" ]; then
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] Загружаем конфигурацию из $CONFIG_FILE"
        source "$CONFIG_FILE"
    else
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] Конфигурационный файл не найден, используем значения по умолчанию"
        create_default_config
    fi
    
    # Создаем необходимые файлы и директории
    create_directories
}

# Создание конфигурационного файла по умолчанию
create_default_config() {
    cat > "$CONFIG_FILE" << EOF
# Конфигурация мониторинга сайтов
# Измените настройки под свои нужды

# Основные настройки
URL_FILE="urls.txt"
LOG_FILE="logs/monitoring.log"
ERROR_LOG="logs/errors.log"
STATUS_FILE="logs/status.json"
CHECK_INTERVAL=5
TIMEOUT=10
MAX_RETRIES=3

# Настройки уведомлений
SEND_EMAIL_ON_ERROR=false
EMAIL_ADDRESS="admin@example.com"
SEND_TELEGRAM_NOTIFICATION=false
TELEGRAM_BOT_TOKEN=""
TELEGRAM_CHAT_ID=""

# Настройки HTTP
USER_AGENT="SiteMonitor/2.0"
FOLLOW_REDIRECTS=true
CHECK_SSL=true

# Настройки отчета
GENERATE_DAILY_REPORT=true
REPORT_FILE="logs/daily_report_\$(date +%Y%m%d).txt"
KEEP_LOGS_DAYS=30
EOF
    chmod 644 "$CONFIG_FILE"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Создан конфигурационный файл по умолчанию: $CONFIG_FILE"
}

# Создание необходимых директорий
create_directories() {
    mkdir -p logs
    mkdir -p reports
    mkdir -p backups
}

# Проверка наличия curl
check_dependencies() {
    if ! command -v curl &> /dev/null; then
        echo "ОШИБКА: curl не установлен!"
        echo "Установите curl:"
        echo "  Ubuntu/Debian: sudo apt-get install curl"
        echo "  CentOS/RHEL:   sudo yum install curl"
        echo "  macOS:         brew install curl"
        exit 1
    fi
    
    if ! command -v jq &> /dev/null; then
        echo "ВНИМАНИЕ: jq не установлен. JSON вывод будет ограничен."
        echo "Установите jq для полного функционала:"
        echo "  Ubuntu/Debian: sudo apt-get install jq"
        echo "  CentOS/RHEL:   sudo yum install jq"
        HAS_JQ=false
    else
        HAS_JQ=true
    fi
}

# Проверка одного URL
check_url() {
    local url=$1
    local url_name=$2
    local retry_count=0
    local http_code=""
    local response_time=""
    local error_msg=""
    local final_status="UNKNOWN"
    
    # Очищаем имя для использования в логах
    local safe_name=$(echo "$url_name" | tr -cd '[:alnum:] _-')
    
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Проверяем: $safe_name ($url)"
    
    # Пытаемся несколько раз при ошибке
    while [ $retry_count -lt $MAX_RETRIES ]; do
        # Используем curl для проверки
        local curl_output
        curl_output=$(curl -s \
            --max-time "$TIMEOUT" \
            --connect-timeout "$TIMEOUT" \
            -w "%{http_code}|%{time_total}" \
            -o /dev/null \
            -L \
            -H "User-Agent: $USER_AGENT" \
            "$url" 2>&1)
        
        local curl_exit_code=$?
        
        if [ $curl_exit_code -eq 0 ]; then
            # Успешный запрос
            http_code=$(echo "$curl_output" | cut -d'|' -f1)
            response_time=$(echo "$curl_output" | cut -d'|' -f2)
            
            # Определяем статус по коду ответа
            if [[ $http_code =~ ^2[0-9]{2}$ ]]; then
                final_status="OK"
                log_result "$safe_name" "$url" "$http_code" "$response_time" "$final_status"
                return 0
            elif [[ $http_code =~ ^3[0-9]{2}$ ]]; then
                final_status="REDIRECT"
                log_result "$safe_name" "$url" "$http_code" "$response_time" "$final_status"
                return 0
            elif [[ $http_code =~ ^4[0-9]{2}$ ]]; then
                final_status="CLIENT_ERROR"
                log_result "$safe_name" "$url" "$http_code" "$response_time" "$final_status"
                return 1
            elif [[ $http_code =~ ^5[0-9]{2}$ ]]; then
                final_status="SERVER_ERROR"
                log_result "$safe_name" "$url" "$http_code" "$response_time" "$final_status"
                return 1
            else
                final_status="UNKNOWN_CODE"
                log_result "$safe_name" "$url" "$http_code" "$response_time" "$final_status"
                return 1
            fi
        else
            # Ошибка curl
            error_msg="$curl_output"
            retry_count=$((retry_count + 1))
            
            if [ $retry_count -lt $MAX_RETRIES ]; then
                echo "[$(date '+%Y-%m-%d %H:%M:%S')] Попытка $retry_count из $MAX_RETRIES не удалась. Повтор через 2 секунды..."
                sleep 2
            fi
        fi
    done
    
    # Все попытки неудачны
    final_status="FAILED"
    log_result "$safe_name" "$url" "000" "0" "$final_status" "$error_msg"
    log_error "$safe_name" "$url" "$error_msg"
    
    # Отправляем уведомление если настроено
    if [ "$SEND_EMAIL_ON_ERROR" = true ] && [ -n "$EMAIL_ADDRESS" ]; then
        send_notification "$safe_name" "$url" "$error_msg"
    fi
    
    return 1
}

# Логирование результата
log_result() {
    local name=$1
    local url=$2
    local code=$3
    local time=$4
    local status=$5
    local error=${6:-""}
    
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local log_entry="[$timestamp] [$status] $name - $url - HTTP: $code - Время: ${time}s"
    
    if [ -n "$error" ]; then
        log_entry="$log_entry - Ошибка: $error"
    fi
    
    # Записываем в основной лог
    echo "$log_entry" >> "$LOG_FILE"
    
    # Выводим в консоль
    if [ "$status" != "OK" ]; then
        echo "  ⚠️  ВНИМАНИЕ: $name недоступен! Статус: $status, Код: $code"
        if [ -n "$error" ]; then
            echo "     Ошибка: $error"
        fi
    else
        echo "  ✓ $name доступен. Код: $code, Время: ${time}s"
    fi
    
    # Обновляем файл статусов
    update_status_file "$name" "$url" "$code" "$time" "$status" "$timestamp"
}

# Логирование ошибки
log_error() {
    local name=$1
    local url=$2
    local error=$3
    
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] $name - $url - $error" >> "$ERROR_LOG"
}

# Обновление файла статусов
update_status_file() {
    if [ "$HAS_JQ" = true ]; then
        local name=$1
        local url=$2
        local code=$3
        local time=$4
        local status=$5
        local timestamp=$6
        
        # Создаем временный файл
        local temp_file="${STATUS_FILE}.tmp"
        
        if [ -f "$STATUS_FILE" ]; then
            # Обновляем существующий файл
            jq --arg name "$name" \
               --arg url "$url" \
               --arg code "$code" \
               --arg time "$time" \
               --arg status "$status" \
               --arg timestamp "$timestamp" \
               '.sites[$name] = {url: $url, code: $code, response_time: $time, status: $status, last_check: $timestamp}' \
               "$STATUS_FILE" > "$temp_file"
        else
            # Создаем новый файл
            echo "{\"last_update\": \"$timestamp\", \"sites\": {}}" | \
            jq --arg name "$name" \
               --arg url "$url" \
               --arg code "$code" \
               --arg time "$time" \
               --arg status "$status" \
               --arg timestamp "$timestamp" \
               '.sites[$name] = {url: $url, code: $code, response_time: $time, status: $status, last_check: $timestamp}' \
               > "$temp_file"
        fi
        
        mv "$temp_file" "$STATUS_FILE"
    fi
}

# Отправка уведомления
send_notification() {
    local name=$1
    local url=$2
    local error=$3
    
    local subject="🚨 Сайт $name недоступен!"
    local message="Сайт: $name
URL: $url
Время: $(date '+%Y-%m-%d %H:%M:%S')
Ошибка: $error
---
Мониторинг сайтов"
    
    # Отправка email (если установлен mailx или sendmail)
    if command -v mailx &> /dev/null; then
        echo "$message" | mailx -s "$subject" "$EMAIL_ADDRESS"
    elif command -v sendmail &> /dev/null; then
        echo "Subject: $subject"$'\n'"$message" | sendmail "$EMAIL_ADDRESS"
    fi
    
    # Можно добавить отправку в Telegram, Slack и т.д.
}

# Проверка всех URL из файла
check_all_urls() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Начинаем проверку всех сайтов"
    echo "========================================"
    
    local total_urls=0
    local success_count=0
    local error_count=0
    
    # Проверяем существование файла с URL
    if [ ! -f "$URL_FILE" ]; then
        echo "ОШИБКА: Файл $URL_FILE не найден!"
        echo "Создайте файл со списком URL в формате:"
        echo "  Название_сайта URL"
        echo "Пример:"
        echo "  Google https://google.com"
        echo "  Яндекс https://ya.ru"
        exit 1
    fi
    
    # Читаем файл построчно
    while IFS= read -r line || [ -n "$line" ]; do
        # Пропускаем пустые строки и комментарии
        line=$(echo "$line" | sed 's/#.*//')
        if [ -z "$line" ]; then
            continue
        fi
        
        # Извлекаем название и URL
        local name=$(echo "$line" | awk '{print $1}')
        local url=$(echo "$line" | awk '{$1=""; print $0}' | sed 's/^ //')
        
        if [ -z "$name" ] || [ -z "$url" ]; then
            echo "ПРЕДУПРЕЖДЕНИЕ: Неверный формат строки: $line"
            continue
        fi
        
        total_urls=$((total_urls + 1))
        
        # Проверяем URL
        if check_url "$url" "$name"; then
            success_count=$((success_count + 1))
        else
            error_count=$((error_count + 1))
        fi
        
        # Небольшая задержка между проверками
        sleep 1
        
    done < "$URL_FILE"
    
    echo "========================================"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Проверка завершена"
    echo "Итого: $total_urls сайтов, Успешно: $success_count, Ошибок: $error_count"
    echo ""
}

# Очистка старых логов
cleanup_old_logs() {
    local keep_days=${KEEP_LOGS_DAYS:-30}
    
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Очищаем логи старше $keep_days дней"
    
    # Очищаем логи
    find logs/ -name "*.log" -mtime +$keep_days -delete 2>/dev/null
    find reports/ -name "*.txt" -mtime +$keep_days -delete 2>/dev/null
    find backups/ -name "*.tar.gz" -mtime +$keep_days -delete 2>/dev/null
}

# Создание отчета
generate_report() {
    local report_file="reports/report_$(date +%Y%m%d_%H%M%S).txt"
    
    echo "=== Отчет мониторинга сайтов ===" > "$report_file"
    echo "Дата: $(date '+%Y-%m-%d %H:%M:%S')" >> "$report_file"
    echo "Проверено сайтов: $(grep -c '^\[.*\] \[.*\] ' "$LOG_FILE" 2>/dev/null || echo "0")" >> "$report_file"
    echo "" >> "$report_file"
    echo "Последние ошибки:" >> "$report_file"
    tail -20 "$ERROR_LOG" 2>/dev/null >> "$report_file" || echo "Ошибок нет" >> "$report_file"
    
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Отчет сохранен в $report_file"
}

# Создание резервной копии
create_backup() {
    local backup_file="backups/monitoring_backup_$(date +%Y%m%d).tar.gz"
    
    tar -czf "$backup_file" "$URL_FILE" "$CONFIG_FILE" "logs/" 2>/dev/null
    
    if [ $? -eq 0 ]; then
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] Резервная копия создана: $backup_file"
    fi
}

# Показать справку
show_help() {
    cat << EOF
Использование: $0 [ОПЦИИ]

Опции:
  -h, --help          Показать эту справку
  -c, --config FILE   Использовать другой конфигурационный файл
  -u, --urls FILE     Использовать другой файл с URL
  -i, --interval MIN  Интервал проверки в минутах
  -o, --once          Выполнить одну проверку и выйти
  -s, --status        Показать текущий статус всех сайтов
  -r, --report        Создать отчет
  -b, --backup        Создать резервную копию
  -v, --verbose       Подробный вывод

Примеры:
  $0                    # Запуск с настройками по умолчанию
  $0 -i 10              # Проверять каждые 10 минут
  $0 -o                 # Выполнить одну проверку
  $0 -c myconfig.conf   # Использовать свой конфиг
  $0 --status           # Показать статус сайтов

Конфигурационный файл: $CONFIG_FILE
Файл с URL: $URL_FILE
Лог-файл: $LOG_FILE
EOF
}

# Показать статус
show_status() {
    if [ -f "$STATUS_FILE" ] && [ "$HAS_JQ" = true ]; then
        echo "=== Текущий статус сайтов ==="
        echo "Последнее обновление: $(jq -r '.last_update' "$STATUS_FILE")"
        echo ""
        
        jq -r '.sites | to_entries[] | 
              "\(.key):\n  URL: \(.value.url)\n  Статус: \(.value.status)\n  Код: \(.value.code)\n  Время: \(.value.response_time)s\n  Проверка: \(.value.last_check)\n"' \
            "$STATUS_FILE"
    elif [ -f "$STATUS_FILE" ]; then
        echo "=== Текущий статус сайтов ==="
        grep -A5 '"sites"' "$STATUS_FILE" || cat "$STATUS_FILE"
    else
        echo "Файл статусов не найден. Запустите проверку сначала."
    fi
}

# ==========================================
# ОСНОВНАЯ ПРОГРАММА
# ==========================================

# Парсинг аргументов
MODE="daemon"
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -c|--config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        -u|--urls)
            URL_FILE="$2"
            shift 2
            ;;
        -i|--interval)
            CHECK_INTERVAL="$2"
            shift 2
            ;;
        -o|--once)
            MODE="once"
            shift
            ;;
        -s|--status)
            MODE="status"
            shift
            ;;
        -r|--report)
            MODE="report"
            shift
            ;;
        -b|--backup)
            MODE="backup"
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        *)
            echo "Неизвестный аргумент: $1"
            show_help
            exit 1
            ;;
    esac
done

# Инициализация
load_config
check_dependencies

# Обработка режимов работы
case "$MODE" in
    "once")
        echo "=== Однократная проверка сайтов ==="
        check_all_urls
        ;;
        
    "status")
        show_status
        ;;
        
    "report")
        generate_report
        ;;
        
    "backup")
        create_backup
        ;;
        
    "daemon")
        echo "=== Мониторинг сайтов ==="
        echo "Конфигурация:"
        echo "  Интервал проверки: $CHECK_INTERVAL минут"
        echo "  Файл с URL: $URL_FILE"
        echo "  Лог  файл: $LOG_FILE"
        echo "  Лог ошибок: $ERROR_LOG"
        echo ""
        echo "Для остановки нажмите Ctrl+C"
        echo ""
        
        # Создаем резервную копию при первом запуске
        create_backup
        
        # Основной цикл
        while true; do
            check_all_urls
            
            # Очищаем старые логи раз в день
            if [ "$(date +%H%M)" = "0000" ]; then
                cleanup_old_logs
                generate_report
            fi
            
            echo "[$(date '+%Y-%m-%d %H:%M:%S')] Следующая проверка через $CHECK_INTERVAL минут..."
            echo ""
            
            # Ждем указанное количество минут
            sleep $((CHECK_INTERVAL * 60))
        done
        ;;
esac
