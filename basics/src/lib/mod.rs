extern crate base64;
extern crate hex;
use std::u8;
use std::str;

use std::collections::HashMap;
pub mod counter;

#[cfg(test)]
mod tests;

pub fn hex_to_base64(hex: String) -> Result<String, hex::FromHexError> {
    let result = hex::decode(&hex);
    match result {
        Ok(bytes) => Ok(base64::encode(&bytes)),
        Err(e) => Err(e)
    }
}

pub fn xor(first: &Vec<u8>, second: &Vec<u8>) -> Result<Vec<u8>, &'static str>  {
    match first.len() == second.len() {
        false => Err("The length of the input vectors should be the same"),
        true => {
            let mut xored = Vec::new();
            let len = first.len(); 
            for i in 0..len {
                xored.push(first[i] ^ second[i]);
            }
            Ok(xored)
        }
    }
}

pub fn xor_hex(first: String, second: String) -> Result<String, &'static str>  {
    let first_bytes =  match hex::decode(&first) {
        Err(_) => return Err("Error in decoding the first hex"),
        Ok(bytes) => bytes
    };
    
    let second_bytes = match hex::decode(&second) {
        Err(_) => return Err("Error in decoding the second hex"),
        Ok(bytes) => bytes 
    };

    let xored = xor(&first_bytes, &second_bytes)?;
    Ok(hex::encode(xored))
}

pub fn xor_single_key(plaintext: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut result = Vec::new();

    for i in 0..plaintext.len() {
        result.push(plaintext[i] ^ key);
    }
    result 
}

pub fn xor_mul_keys(plaintext: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    for i in 0..plaintext.len() {
        result.push(plaintext[i] ^ key[i % key.len()]);
    }
    result 
}

pub fn get_frequency_table() -> HashMap<&'static str, f64> {
    // https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
    let mut hm = HashMap::from([
        ("E",	11.1607),	("M",	3.0129),
        ("A",	8.4966),	("H",	3.0034),
        ("R",	7.5809),	("G",	2.4705),
        ("I",	7.5448),	("B",	2.0720),
        ("O",	7.1635),	("F",	1.8121),
        ("T",	6.9509),	("Y",	1.7779),
        ("N",	6.6544),	("W",	1.2899),
        ("S",	5.7351),	("K",	1.1016),
        ("L",	5.4893),	("V",	1.0074),
        ("C",	4.5388),	("X",	0.2902),
        ("U",	3.6308),	("Z",	0.2722),
        ("D",	3.3844),	("J",	0.1965),
        ("P",	3.1671),	("Q",	0.1962)
    ]);
    let punteggiatura = [".", " ", ",", "'"];
    for ch in punteggiatura {
        hm.insert(ch, 2.0);
    }

    hm
}


pub fn frequency_attack(ciphertext: &Vec<u8>) -> (u8, f64) {
    // TODO: make a struct for this return type, so it has better meaning
    let mut scores: [f64; 256] = [0.0; 256];
    for i in 0..=255 {
        let plaintext = xor_single_key(&ciphertext, i.clone());
        scores[i as usize] = get_score(&plaintext);
    }
    
    let mut max = -1.0;
    let mut index = 0;
    scores.iter().enumerate().for_each(|(i, value)| {
        if *value > max {
            max = *value;
            index = i;
        }
    });
    (index as u8, max as f64)
}

pub fn get_score(maybe_plaintext: &Vec<u8>) -> f64 {
    // TODO: make this return Result, with error enum that has entries for the table and utf8
    use counter::Counter;
    let table = get_frequency_table();
    let counter: Counter<u8> = Counter::from(&maybe_plaintext);

    let mut score: f64 = 0.0;
    for byte in maybe_plaintext {
        let uppercase = [byte.to_ascii_uppercase()];
        if let Ok(ch) = str::from_utf8(&uppercase){
            if let Some(value) = table.get(ch.clone()) {
                let shannons = (counter.total() as f64) / (counter.get(&byte).unwrap() as f64);
                score += value * shannons.log2();
            }
        }
    }

    score 
}
