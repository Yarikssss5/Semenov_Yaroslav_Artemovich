use std::collections::HashSet;
use std::fs;
use std::io;

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=use+std%3A%3Acollections%3A%3AHashSet%3B%0A%0Afn+decode_str%28key%3A+%26str%2C+msg%3A+%26str%2C+alphavet_set%3A+%26HashSet%3Cchar%3E%2C+alphavet%3A+%26Vec%3Cchar%3E%29+-%3E+String+%7B%0A++++let+mut+out%3A+String+%3D+String%3A%3Anew%28%29%3B%0A++++let+key_chars%3A+Vec%3Cchar%3E+%3D+key.chars%28%29.collect%28%29%3B%0A++++let+mut+position_in_key%3A+usize+%3D+0%3B%0A++++for+i+in+msg.chars%28%29+%7B%0A++++++++if+position_in_key+%3E%3D+key.len%28%29+%7B+position_in_key+%3D+0%3B+%7D%0A++++++++if+%21alphavet_set.contains%28%26i%29+%7B%0A++++++++++++out.push%28i%29%3B%0A++++++++++++continue%3B%0A++++++++%7D%0A++++++++let+key_char%3A+char+%3D+key_chars%5Bposition_in_key+%25+key_chars.len%28%29%5D%3B%0A++++++++let+msg_pos%3A+usize+%3D+alphavet.iter%28%29.position%28%7C%26c%7C+c+%3D%3D+i%29.unwrap%28%29%3B%0A++++++++let+key_pos%3A+usize+%3D+alphavet.iter%28%29.position%28%7C%26c%7C+c+%3D%3D+key_char%29.unwrap%28%29%3B%0A++++++++let+new_pos%3A+usize+%3D+if+msg_pos+%3E%3D+key_pos+%7B%0A++++++++++++msg_pos+-+key_pos%0A++++++++%7D+else+%7B%0A++++++++++++alphavet.len%28%29+-+%28key_pos+-+msg_pos%29%0A++++++++%7D%3B%0A++++++++out.push%28alphavet%5Bnew_pos%5D%29%3B%0A++++++++position_in_key+%2B%3D+1%3B%0A++++%7D%0A++++out%0A%7D%0A%0A%0Afn+main%28%29+%7B%0A++++let+alphabet%3A+Vec%3Cchar%3E+%3D+%7B%0A++++++++let+mut+alphabet_vec+%3D+Vec%3A%3Anew%28%29%3B%0A++++++++%2F%2F+%D0%94%D0%BE%D0%B1%D0%B0%D0%B2%D0%BB%D1%8F%D0%B5%D0%BC+%D0%B1%D1%83%D0%BA%D0%B2%D1%8B+%D0%B0-%D0%B5%0A++++++++for+c+in+%27%D0%B0%27..%3D%27%D1%8F%27+%7B%0A++++++++++++alphabet_vec.push%28c%29%3B%0A++++++++++++if+c+%3D%3D+%27%D0%B5%27+%7B%0A++++++++++++++++alphabet_vec.push%28%27%D1%91%27%29%3B%0A++++++++++++%7D%0A++++++++%7D%0A++++++++alphabet_vec%0A++++%7D%3B%0A++++let+alphabet_set%3A+HashSet%3Cchar%3E+%3D+alphabet.iter%28%29.copied%28%29.collect%28%29%3B%0A++++println%21%28%22%7B%7D%22%2C+decode_str%28%22%D0%BA%D0%BD%D0%B8%D0%B3%D0%B0%22%2C+%22%D1%8D%D1%82%D1%8A%D1%85%D0%BE%D0%BC%D0%BD%D0%B7+%D1%84%D1%82%D1%8B%D1%8C%D1%83%D0%B3+alt%22%2C+%26alphabet_set%2C+%26alphabet%29%29%3B%0A%7D

fn encode_str(key: &str, msg: &str, alphabet_set: &HashSet<char>, alphabet: &[char]) -> String {
    let key_chars: Vec<char> = key.chars().collect();
    let mut key_iter = key_chars.iter().cycle(); // Используем бесконечный итератор
    msg.chars()
        .map(|msg_char| {
            if !alphabet_set.contains(&msg_char) {
                return msg_char;
            }
            let key_char: &char = key_iter.next().unwrap(); // Всегда будет Some, т.к. cycle()
            let msg_pos: usize = alphabet.iter().position(|&c: &char| c == msg_char)
                .unwrap_or_else(|| panic!("Character '{}' not in alphabet", msg_char));
            let key_pos: usize = alphabet.iter().position(|&c: &char| c == *key_char)
                .unwrap_or_else(|| panic!("Key character '{}' not in alphabet", key_char));
            let new_pos: usize = (msg_pos + key_pos) % alphabet.len();
            alphabet[new_pos]
        }).collect()
}

fn decode_str(key: &str, msg: &str, alphavet_set: &HashSet<char>, alphavet: &Vec<char>) -> String {
    let mut out: String = String::new();
    let key_chars: Vec<char> = key.chars().collect();
    let mut position_in_key: usize = 0;
    for i in msg.chars() {
        if position_in_key >= key.len() { position_in_key = 0; }
        if !alphavet_set.contains(&i) {
            out.push(i);
            continue;
        }
        let key_char: char = key_chars[position_in_key % key_chars.len()];
        let msg_pos: usize = alphavet.iter().position(|&c| c == i).unwrap();
        let key_pos: usize = alphavet.iter().position(|&c| c == key_char).unwrap();
        let new_pos: usize = if msg_pos >= key_pos {
            msg_pos - key_pos
        } else {
            alphavet.len() - (key_pos - msg_pos)
        };
        out.push(alphavet[new_pos]);
        position_in_key += 1;
    }
    out
}

fn main() -> io::Result<()> {
    // Создаем русский алфавит (33 буквы, включая ё)
    let alphabet: Vec<char> = {
        let mut alphabet_vec = Vec::new();
        // Добавляем буквы а-е
        for c in 'а'..='я' {
            alphabet_vec.push(c);
            if c == 'е' {
                alphabet_vec.push('ё');
            }
        }
        alphabet_vec
    };
    // Создаем HashSet для быстрой проверки принадлежности к алфавиту
    let alphabet_set: HashSet<char> = alphabet.iter().copied().collect();
    // Читаем сообщение из файла 1.txt
    let message = fs::read_to_string("1.txt").map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Не удалось прочитать файл 1.txt: {}", e),
        )
    })?;
    // Читаем ключ из файла 2.txt
    let key = fs::read_to_string("2.txt").map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Не удалось прочитать файл 2.txt: {}", e),
        )
    })?;
    // Убираем пробелы и переводы строк из ключа
    let key = key.trim();
    // Шифруем сообщение
    let encrypted_message = encode_str(key, &message, &alphabet_set, &alphabet);
    // Записываем результат в файл 3.txt
    fs::write("3.txt", &encrypted_message)?;
    println!("Шифрование завершено успешно!");
    println!("Исходное сообщение из 1.txt: {}", message.trim());
    println!("Ключ из 2.txt: {}", key);
    println!(
        "Зашифрованное сообщение записано в 3.txt: {}",
        encrypted_message.trim()
    );
    Ok(())
}
