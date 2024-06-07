use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use clap;
use clap::Parser;
use utils::generate_string_with_random_chars;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    pub file_path: String,
}

fn count_chars_in_file<P: AsRef<Path>>(file_path: P) -> io::Result<HashMap<char, usize>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    count_chars(contents.chars())
}

fn count_chars(contents: impl IntoIterator<Item = char>) -> io::Result<HashMap<char, usize>> {
    let mut frequencies = HashMap::new();
    for c in contents {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    Ok(frequencies)
}

fn calculate_entropy(frequencies: &HashMap<char, usize>) -> f64 {
    let total_chars = frequencies.values().sum::<usize>() as f64;
    frequencies.values().fold(0.0, |entropy, &frequency| {
        let probability = frequency as f64 / total_chars;
        entropy - probability * probability.log2()
    })
}


fn main() -> io::Result<()> {

    let args = Cli::parse();

    let frequencies = count_chars_in_file(args.file_path)?;
    // for (c, count) in &frequencies {
    //     println!("'{}': {}", c, count);
    // }

    let entropy = calculate_entropy(&frequencies);
    println!("Entropy: {}", entropy);

    let random_unary = generate_string_with_random_chars(1000, 1);
    let random_binary = generate_string_with_random_chars(1000, 2);
    let random_256 = generate_string_with_random_chars(1000, 255);

    let unary_frequencies = count_chars(random_unary.chars())?;
    let unary_entropy = calculate_entropy(&unary_frequencies);
    println!("Entropy of unary string: {}\nlog2(n) = {}", unary_entropy, 1.0f64.log2());

    let binary_frequencies = count_chars(random_binary.chars())?;
    let binary_entropy = calculate_entropy(&binary_frequencies);
    println!("Entropy of binary string: {}\nlog2(n) = {}", binary_entropy, 2.0f64.log2());

    let _256_frequencies = count_chars(random_256.chars())?;
    let _256_entropy = calculate_entropy(&_256_frequencies);
    println!("Entropy of 256-character string: {}\nlog2(n) = {}", _256_entropy, 255.0f64.log2());

    Ok(())
}