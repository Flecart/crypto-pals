

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn xor_single_key() {
    use hex;
    let input = "Sono un testo per testare il funzionamento".to_string().into_bytes();
    let bytes = super::xor_single_key(&input, 10);
    println!("{}", hex::encode(&bytes));
    assert!("596564652a7f642a7e6f797e652a7a6f782a7e6f797e6b786f2a63662a6c7f64706365646b676f647e65".eq(&hex::encode(&bytes)))
}

#[test]
fn hex_to_base64() {
    use super::hex_to_base64;

    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", hex_to_base64(input.to_string()).unwrap());
}

#[test]
fn xor_hex() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";

    let test = "746865206b696420646f6e277420706c6179";

    let ouput = super::xor_hex(a.to_string(), b.to_string()).unwrap();
    println!("{}", ouput);
    assert!(test.eq(&ouput));
}

#[test]
fn char_str_in_freqtable() {
    use std::str::from_utf8;
    let table = super::get_frequency_table();
    let character: u8 = 97u8;
    let ch = [character.to_ascii_uppercase()];
    println!("ascii: {}", from_utf8(&ch).unwrap());
    println!("table: {:?}", table);

    let freq = table.get(from_utf8(&ch).unwrap()).unwrap();
    println!("value: {:?}", freq);

    assert!( from_utf8(&ch).unwrap() == "A");
    assert!(*freq == 8.4966);
}

#[test]
fn get_score() {
    // Test dependant on freqtable!
    let test = vec![97u8, 98u8];
    let score = super::get_score(&test);
    println!("value: {}, test: {}", score, 8.4966 * 2_f64.log2() + 2.0720 * 2_f64.log2());
    
    assert!(8.4966 * 2_f64.log2() + 2.0720 * 2_f64.log2() == score);
}

#[test]
fn counter() {
    use super::counter;
    let hello = vec![1,2,3,4,5,6,7,8,9,1,2,1];
    let c = counter::Counter::from(&hello);
    assert!(c.total() == 12);
    assert!(c.get(&1).unwrap() == 3);
    assert!(c.get(&2).unwrap() == 2);
    assert!(c.get(&3).unwrap() == 1);
}

#[test]
fn freq_attack() {
    let text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex::decode(text).unwrap();
    let key = super::frequency_attack(&bytes).0;

    let plaintext_bytes = super::xor_single_key(&bytes, key);
    let plaintext = std::str::from_utf8(&plaintext_bytes).unwrap();
    println!("Text: {}", plaintext.to_string());
    assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
}

#[test]
fn detect_single_key() {
    use std::time::Instant;
    let now = Instant::now();
    
    let mut string: Vec<u8> = Vec::new();
    let mut best_score: (u8, f64) = (0, 0.0);
    if let Ok(lines) = read_lines("./4_chal.data") {
        for line_res in lines {
            if let Ok(line) = line_res {
                let bytes = hex::decode(line).unwrap();
                let score = super::frequency_attack(&bytes);
                if score.1 > best_score.1 {
                    best_score = score.clone();
                    string = bytes.clone();
                }
            }
        }
    }

    let plaintext_bytes = super::xor_single_key(&string, best_score.0);
    let plaintext = std::str::from_utf8(&plaintext_bytes).unwrap();
    println!("Text: {}", plaintext.to_string());
    assert_eq!("Now that the party is jumping\n", plaintext);
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[test]
fn xor_mul_keys() {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let test = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    let input_bytes = input.to_string().into_bytes();
    let key: Vec<u8> = vec![b'I', b'C', b'E'];
    let output = super::xor_mul_keys(&input_bytes, &key);
    let output_hex = hex::encode(&output);
    assert_eq!(output_hex, test);
}

#[test]
fn hamming_distance() {
    let first = "this is a test".to_string();
    let second = "wokka wokka!!!".to_string();

    let hd = super::hamming_distance(&first.into_bytes(), &second.into_bytes()).unwrap();
    assert_eq!(hd, 37);
}