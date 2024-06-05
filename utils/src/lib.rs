use rand;

pub fn generate_string_with_random_chars(num_chars: usize, alphabet_size: u8) -> String {
    (0..num_chars)
        .map(|_| rand::random::<u8>() % alphabet_size)
        .map(|c| (c + b'a') as char)
        .collect()
}