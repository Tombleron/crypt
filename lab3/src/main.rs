use std::collections::VecDeque;
use std::io::{self, Read, Write};
use utils::generate_string_with_random_chars;
use rc4;

fn generate_key(size: usize) -> String {
    let key = generate_string_with_random_chars(size, 26);

    println!("Key is:\n\t{}", key);

    key
}

fn vernam_cipher(mut input_text: String, key: String) {
    if key.len() < input_text.len() {
        println!("Key is too short.\nTruncating input text to match key length.");
        input_text = input_text[..key.len()].to_string();
    }

    let mut output = String::new();
    for (a, b) in input_text.chars().zip(key.chars()) {
        let c = (a as u8 ^ b as u8);
        output.push(c as char);
    }

    println!("Result: {}", output);
}

pub struct RC4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl RC4 {
    pub fn new(key: &[u8]) -> Self {
        let mut s = [0u8; 256];
        for i in 0..256 {
            s[i] = i as u8;
        }

        let mut j = 0;
        for i in 0..256 {
            j = (j + s[i] as usize + key[i % key.len()] as usize) & 0xFF;
            s.swap(i, j as usize);
        }

        RC4 { s, i: 0, j: 0 }
    }

    pub fn process(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            self.i = (self.i + 1) & 0xFF;
            self.j = (self.j + self.s[self.i as usize]) & 0xFF;
            self.s.swap(self.i as usize, self.j as usize);

            let k = self.s[((self.s[self.i as usize] as u16 + self.s[self.j as usize] as u16) & 0xFF) as usize];
            *byte ^= k;
        }
    }
}

fn main() {
    let key = generate_key(1000);

    loop {
        println!("1. Vernam cipher");
        println!("2. RC4");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let Ok(choice): Result<u8, _> = dbg!(choice.trim()).parse() else {
            continue;
        };

        match choice {
            1 => {
                let mut input_text = String::new();
                println!("Enter text to encrypt:");
                io::stdin().read_line(&mut input_text).unwrap();

                vernam_cipher(input_text, key.clone());
            }
            2 => {
                let mut input_text = String::new();
                println!("Enter text to encrypt:");
                io::stdin().read_line(&mut input_text).unwrap();

                let mut rc4 = RC4::new(key.as_bytes());
                let mut input_bytes = input_text.as_bytes().to_vec();
                rc4.process(&mut input_bytes);
                println!("Encrypted text: {:?}", input_bytes);

                let mut rc4 = RC4::new(key.as_bytes());
                rc4.process(&mut input_bytes);
                let result = String::from_utf8(input_bytes).unwrap();
                println!("Decrypted text: {}", result);

            }
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}
