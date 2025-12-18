use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, ScrolledWindow,
    TextBuffer, TextView,
};
use std::cell::RefCell;
use std::rc::Rc;

fn xor_encrypt(text: &str, key: &str) -> String {
    let text_bytes = text.as_bytes();
    let key_bytes = key.as_bytes();

    if key_bytes.is_empty() {
        return text.to_string();
    }

    let result: Vec<u8> = text_bytes
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_bytes.len()])
        .collect();

    // Для отображения в шестнадцатеричном формате
    result
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

fn xor_decrypt(hex_str: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();

    if key_bytes.is_empty() {
        return hex_str.to_string();
    }

    // Преобразуем hex строку обратно в байты
    let mut bytes = Vec::new();
    for i in (0..hex_str.len()).step_by(2) {
        if i + 1 < hex_str.len() {
            if let Ok(byte) = u8::from_str_radix(&hex_str[i..i + 2], 16) {
                bytes.push(byte);
            }
        }
    }

    // Применяем XOR с ключом
    let result: Vec<u8> = bytes
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_bytes.len()])
        .collect();

    String::from_utf8_lossy(&result).to_string()
}

fn build_ui(application: &gtk::Application) {
    // Создаем главное окно
    let window = ApplicationWindow::new(application);
    window.set_title("XOR Encryptor/Decryptor");
    window.set_default_size(500, 400);
    window.set_position(gtk::WindowPosition::Center);

    // Основной вертикальный контейнер
    let main_box = Box::new(Orientation::Vertical, 5);
    main_box.set_margin_top(10);
    main_box.set_margin_bottom(10);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    // Поле для ввода ключа
    let key_label = Label::new(Some("Ключ шифрования:"));
    main_box.add(&key_label);

    let key_entry = Entry::new();
    key_entry.set_placeholder_text(Some("Введите ключ..."));
    main_box.add(&key_entry);

    // Текст для шифрования
    let input_label = Label::new(Some("Исходный текст:"));
    main_box.add(&input_label);

    let input_scroll = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    input_scroll.set_min_content_height(100);

    let input_textview = TextView::new();
    input_scroll.add(&input_textview);
    main_box.add(&input_scroll);

    // Кнопки
    let button_box = Box::new(Orientation::Horizontal, 5);

    let encrypt_button = Button::with_label("Зашифровать");
    let decrypt_button = Button::with_label("Расшифровать");
    let clear_button = Button::with_label("Очистить");

    button_box.add(&encrypt_button);
    button_box.add(&decrypt_button);
    button_box.add(&clear_button);
    main_box.add(&button_box);

    // Результат
    let result_label = Label::new(Some("Результат:"));
    main_box.add(&result_label);

    let result_scroll = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    result_scroll.set_min_content_height(100);

    let result_textview = TextView::new();
    result_textview.set_editable(false);
    result_textview.set_wrap_mode(gtk::WrapMode::Word);
    result_scroll.add(&result_textview);
    main_box.add(&result_scroll);

    // Получаем буферы для текста
    let input_buffer = input_textview
        .buffer()
        .expect("Не удалось получить буфер ввода");
    let result_buffer = result_textview
        .buffer()
        .expect("Не удалось получить буфер результата");

    // Клонируем переменные для использования в замыканиях
    let key_entry_rc = Rc::new(RefCell::new(key_entry));
    let input_buffer_rc = Rc::new(RefCell::new(input_buffer));
    let result_buffer_rc = Rc::new(RefCell::new(result_buffer));

    // Обработчик кнопки шифрования
    {
        let key_entry_clone = key_entry_rc.clone();
        let input_buffer_clone = input_buffer_rc.clone();
        let result_buffer_clone = result_buffer_rc.clone();

        encrypt_button.connect_clicked(move |_| {
            let key = key_entry_clone.borrow().text().to_string();
            let input_text = {
                let buffer = input_buffer_clone.borrow();
                buffer
                    .text(&buffer.start_iter(), &buffer.end_iter(), true)
                    .unwrap_or_default()
            };

            if !input_text.is_empty() {
                let encrypted = xor_encrypt(&input_text, &key);
                result_buffer_clone.borrow().set_text(&encrypted);
            }
        });
    }

    // Обработчик кнопки расшифрования
    {
        let key_entry_clone = key_entry_rc.clone();
        let input_buffer_clone = input_buffer_rc.clone();
        let result_buffer_clone = result_buffer_rc.clone();

        decrypt_button.connect_clicked(move |_| {
            let key = key_entry_clone.borrow().text().to_string();
            let input_text = {
                let buffer = input_buffer_clone.borrow();
                buffer
                    .text(&buffer.start_iter(), &buffer.end_iter(), true)
                    .unwrap_or_default()
            };

            if !input_text.is_empty() {
                let decrypted = xor_decrypt(&input_text, &key);
                result_buffer_clone.borrow().set_text(&decrypted);
            }
        });
    }

    // Обработчик кнопки очистки
    {
        let input_buffer_clone = input_buffer_rc.clone();
        let result_buffer_clone = result_buffer_rc.clone();

        clear_button.connect_clicked(move |_| {
            input_buffer_clone.borrow().set_text("");
            result_buffer_clone.borrow().set_text("");
        });
    }

    // Добавляем контейнер в окно и отображаем все
    window.add(&main_box);
    window.show_all();
}

fn main() {
    // Создаем приложение GTK
    let application = Application::new(Some("com.example.xor-encryptor"), Default::default());
    application.connect_activate(|app| {
        build_ui(app);
    });
    // Запускаем приложение
    application.run();
}
