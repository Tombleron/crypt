use std::collections::HashSet;
use std::io::{self, BufRead, Write};
use std::iter::{IntoIterator, Iterator};

fn count_words_in_dictionary(text: &str) -> usize {
    let dict: HashSet<String> = [
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "I",
        "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
        "this", "but", "his", "by", "from", "they", "we", "say", "her",
        "she", "or", "an", "will", "my", "one", "all", "would", "there",
        "their", "what", "so", "up", "out", "if", "about", "who", "get",
        "which", "go", "me", "when", "make", "can", "like", "time", "no",
        "just", "him", "know", "take", "people", "into", "year", "your",
        "good", "some", "could", "them", "see", "other", "than", "then",
        "now", "look", "only", "come", "its", "over", "think", "also",
        "back", "after", "use", "two", "how", "our", "work", "first",
        "well", "way", "even", "new", "want", "because", "any", "these",
        "give", "day", "most", "us"
    ].into_iter().map(|s| s.to_owned()).collect();

    text.split_whitespace()
        .map(|word| word.to_ascii_lowercase())
        .filter(|word| dict.contains(word))
        .count()
}

fn caesar_encrypt(plaintext: &str, key: u8) -> String {
    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let a = if c.is_ascii_lowercase() { 'a' } else { 'A' };
                let shift = (c as u8 - a as u8 + key) % 26;
                (a as u8 + shift) as char
            } else {
                c
            }
        })
        .collect()
}

fn find_key(plaintext: &str, ciphertext: &str) -> u8 {
    let plaintext_char = plaintext.chars().next().unwrap().to_ascii_lowercase();
    let ciphertext_char = ciphertext.chars().next().unwrap().to_ascii_lowercase();

    if ciphertext_char >= plaintext_char {
        ciphertext_char as u8 - plaintext_char as u8
    } else {
        ciphertext_char as u8 + 26 - plaintext_char as u8
    }
}

fn caesar_decrypt(ciphertext: &str, key: u8) -> String {
    caesar_encrypt(ciphertext, 26 - key)
}

fn main() {
    loop {
        let mut choice = String::new();

        println!("What do you want to do?");
        println!("1. Encrypt text");
        println!("2. Find key");
        println!("3. Decrypt all");
        println!("4. Decrypt likely");
        println!("5. Exit");

        io::stdin().read_line(&mut choice).unwrap();
        let Ok(choice) = choice.trim().parse() else {
            println!("Invalid choice");
            continue;
        };

        match choice {
            1 => {
                let mut plaintext = String::new();
                let mut key_string = String::new();

                print!("Enter plaintext: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut plaintext).unwrap();

                print!("Enter key: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut key_string).unwrap();
                let key: u8 = key_string.trim().parse().unwrap();

                let ciphertext = caesar_encrypt(&plaintext.trim(), key);
                println!("Ciphertext: {}", ciphertext);
            },
            2 => {
                let mut plaintext = String::new();
                let mut ciphertext = String::new();

                print!("Enter plaintext: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut plaintext).unwrap();

                print!("Enter ciphertext: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut ciphertext).unwrap();

                let key = find_key(&plaintext.trim(), &ciphertext.trim());
                println!("Key: {}", key);
            },
            3 => {
                let mut ciphertext = String::new();

                print!("Enter ciphertext: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut ciphertext).unwrap();

                for key in 0..26 {
                    let plaintext = caesar_decrypt(&ciphertext.trim(), key);
                    println!("Key [{}]\n\t{}", key, plaintext);
                }
            },
            4 => {
                let mut ciphertext = String::new();

                print!("Enter ciphertext: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut ciphertext).unwrap();

                let mut max_words = 0;
                let mut likely_key = 0;

                for key in 0..26 {
                    let plaintext = caesar_decrypt(&ciphertext.trim(), key);
                    let words = count_words_in_dictionary(&plaintext);
                    if words > max_words {
                        max_words = words;
                        likely_key = key;
                    }
                }

                println!("Likely key: {}", likely_key);
                println!("Decrypted text: {}", caesar_decrypt(&ciphertext.trim(), likely_key));
            },
            5 => break,
            _ => println!("Invalid choice"),
        }
    }
}