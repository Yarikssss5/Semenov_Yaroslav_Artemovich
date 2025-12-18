use std::fs;
use std::io;

const SHIFT: i32 = 3;
const ALPHABET_SIZE: i32 = 32; // Количество букв в русском алфавите

fn caesar_cipher_char(ch: char, shift: i32) -> char {
    if ('а'..='я').contains(&ch) {
        let base = 'а' as i32;
        let current = ch as i32;
        let shifted_code = base + (current - base + shift).rem_euclid(ALPHABET_SIZE);
        char::from_u32(shifted_code as u32).unwrap_or(ch)
    } else {
        ch
    }
}

fn caesar_cipher_text(text: &str, shift: i32) -> String {
    text.chars()
        .map(|ch| caesar_cipher_char(ch, shift))
        .collect()
}

fn main() -> io::Result<()> {
    // Проверяем существование входного файла
    if !fs::metadata("1.txt").is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Файл 1.txt не найден. Создайте его с сообщением для шифрования.",
        ));
    }

    // Читаем входной файл
    let input_text = fs::read_to_string("1.txt")?;

    // Шифруем текст
    let encrypted_text = caesar_cipher_text(&input_text, SHIFT);

    // Записываем результат
    fs::write("2.txt", encrypted_text)?;

    println!("✅ Шифрование завершено!");
    println!("📄 Исходный текст сохранен в 1.txt");
    println!("🔒 Зашифрованный текст сохранен в 2.txt");

    Ok(())
}
