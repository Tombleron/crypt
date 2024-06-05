use aes_gcm::{aead::{Aead, KeyInit, OsRng}, AeadCore, Aes256Gcm, Key, Nonce};
use std::io::{self, Write};

fn main() {
    // Запрашиваем у пользователя ввод строки для шифрования
    println!("Введите строку для шифрования:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let input_string = input_string.trim().to_string();  // Удаляем лишние пробелы и символы новой строки

    // Задаем ключ (32 символа)
    let key_str = "thiskeystrmustbe32charlongtowork".to_string();
    println!("Ключ: {}", key_str.clone());

    // Шифруем строку
    let encrypted_data = encrypt(key_str.clone(), input_string);
    println!("Зашифрованные данные: {:?}", encrypted_data.clone());

    // Расшифровываем строку
    let decrypted_string = decrypt(key_str, encrypted_data);
    println!("Расшифрованная строка: {}", decrypted_string);
}

fn encrypt(key_str: String, plaintext: String) -> String {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    println!("Nonce: {:?}", nonce.clone());

    let cipher = Aes256Gcm::new(key);

    let ciphered_data = cipher.encrypt(&nonce, plaintext.as_bytes())
        .expect("failed to encrypt");

    // Объединяем nonce и зашифрованные данные вместе для хранения
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    hex::encode(encrypted_data)
}

fn decrypt(key_str: String, encrypted_data: String) -> String {
    let encrypted_data = hex::decode(encrypted_data)
        .expect("failed to decode hex string into vec");

    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());

    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);

    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher.decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data");

    String::from_utf8(plaintext)
        .expect("failed to convert vector of bytes to string")
}