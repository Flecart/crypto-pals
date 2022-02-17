pub mod lib;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    first_single();
}

fn first_multiple() {
    
    use std::time::Instant;
    let now = Instant::now();
    
    let mut strings: Vec<Vec<u8>> = Vec::new();
    let mut scores: Vec<(u8, f64)> = Vec::new();
    if let Ok(lines) = read_lines("./4_chal.data") {
        for line_res in lines {
            if let Ok(line) = line_res {
                let bytes = hex::decode(line).unwrap();
                strings.push(bytes.clone());
                scores.push(lib::frequency_attack(&bytes));
            }
        }
    }

    let mut key_score = (0, 0.0);
    let mut index = 0;
    scores.iter().enumerate().for_each(|(i, val)| {
        if val.1 > key_score.1 {
            key_score = *val;
            index = i;
        }
    });
    let plaintext_bytes = lib::xor_single_key(&strings[index], key_score.0);
    let plaintext = std::str::from_utf8(&plaintext_bytes).unwrap();
    println!("Text: {}", plaintext.to_string());

    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}


fn first_single() {
    
    use std::time::Instant;
    let now = Instant::now();
    
    let mut string: Vec<u8> = Vec::new();
    let mut best_score: (u8, f64) = (0, 0.0);
    if let Ok(lines) = read_lines("./4_chal.data") {
        for line_res in lines {
            if let Ok(line) = line_res {
                let bytes = hex::decode(line).unwrap();
                let score = lib::frequency_attack(&bytes);
                if score.1 > best_score.1 {
                    best_score = score.clone();
                    string = bytes.clone();
                }
            }
        }
    }

    let plaintext_bytes = lib::xor_single_key(&string, best_score.0);
    let plaintext = std::str::from_utf8(&plaintext_bytes).unwrap();
    println!("Text: {}", plaintext.to_string());

    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}