use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::io::{BufRead, BufReader, BufWriter};

const MAX_ATTEMPTS: u8 = 3;
const CORRECT_PASSWORD: &str = "secure123"; // Пароль можно изменить
const FILENAME: &str = "protected_file.txt";

fn main() {
    // Алгоритм:
    // 1. Запросить пароль для входа в программу
    // 2. Проверять пароль до 3 попыток
    // 3. При успешном вводе - предоставить доступ к файлу
    // 4. При превышении попыток - завершить программу

    println!("=== ЗАЩИТА ФАЙЛА ПАРОЛЕМ ===");

    let mut attempts = 0;
    let mut authenticated = false;

    // Цикл для ввода пароля (максимум 3 попытки)
    while attempts < MAX_ATTEMPTS {
        println!("\nВВЕДИТЕ ПАРОЛЬ ДЛЯ ВХОДА В ПРОГРАММУ:");

        let mut input = String::new();
        io::stdout().flush().unwrap(); // Очистка буфера вывода

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input_password = input.trim();

                // Условный оператор для проверки пароля
                if input_password == CORRECT_PASSWORD {
                    authenticated = true;
                    break; // Выход из цикла при успешной аутентификации
                } else {
                    attempts += 1;
                    let remaining_attempts = MAX_ATTEMPTS - attempts;

                    if attempts < MAX_ATTEMPTS {
                        println!("ПАРОЛЬ НЕВЕРНЫЙ! ИСПОЛЬЗУЙТЕ ЕЩЕ ОДНУ ПОПЫТКУ");
                        println!("Осталось попыток: {}", remaining_attempts);
                    }
                }
            }
            Err(e) => {
                println!("Ошибка чтения ввода: {}", e);
                attempts += 1;
            }
        }
    }

    // Условный оператор для определения дальнейших действий
    if authenticated {
        println!("\nДОБРО ПОЖАЛОВАТЬ!");
        access_protected_file();
    } else {
        println!("\nВЫ ПРЕВЫСИЛИ ДОПУСТИМОЕ ЧИСЛО ПОПЫТОК! ДО СВИДАНИЯ!");
        println!("Программа завершена.");
    }
}

fn access_protected_file() {
    println!("\n=== РАБОТА С ЗАЩИЩЕННЫМ ФАЙЛОМ ===");

    // Выбор действия с файлом
    loop {
        println!("\nВыберите действие:");
        println!("1. Просмотреть содержимое файла");
        println!("2. Добавить текст в файл");
        println!("3. Выйти из программы");

        print!("Ваш выбор (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => view_file(),
            "2" => add_to_file(),
            "3" => {
                println!("Выход из программы...");
                break;
            }
            _ => println!("Неверный выбор. Попробуйте снова."),
        }
    }
}

fn view_file() {
    match File::open(FILENAME) {
        Ok(file) => {
            let reader = BufReader::new(file);
            println!("\n=== СОДЕРЖИМОЕ ФАЙЛА ===");

            for (i, line) in reader.lines().enumerate() {
                match line {
                    Ok(content) => println!("{}: {}", i + 1, content),
                    Err(e) => println!("Ошибка чтения строки: {}", e),
                }
            }
        }
        Err(_) => {
            println!("Файл не найден. Создайте новый файл, добавив в него текст.");
        }
    }
}

fn add_to_file() {
    println!("\nВведите текст для добавления в файл:");

    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();

    match OpenOptions::new().create(true).append(true).open(FILENAME) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            use std::io::Write;

            if let Err(e) = writeln!(writer, "{}", text.trim()) {
                println!("Ошибка записи в файл: {}", e);
            } else {
                println!("Текст успешно добавлен в файл.");
            }
        }
        Err(e) => {
            println!("Ошибка открытия файла: {}", e);
        }
    }
}
