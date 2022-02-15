extern crate base64;
extern crate hex;
use std::u8;

use std::collections::HashMap;
mod counter;

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

pub fn xor_single_key(plaintext: String, key: u8) -> Vec<u8> {
    let bytes = plaintext.into_bytes();
    let mut result = Vec::new();

    for i in 0..bytes.len() {
        result.push(bytes[i] ^ key)
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


pub fn frequency_attack(ciphertext: String) -> u8 {
    let mut scores: [f64; 256];

    for i in 0..256 {
        let plaintext = xor_single_key(ciphertext, i);
        scores[i] = get_score(plaintext); // TODO: solve index error
    }
    0 // TODO: scegli il migliore fra gli score
}

pub fn get_score(maybe_plaintext: Vec<u8>) -> f64 {
    use counter::Counter;
    let table = get_frequency_table();
    let counter: Counter<u8> = Counter::from(&maybe_plaintext);

    let mut score = 0;
    for byte in maybe_plaintext {
        // TODO
    }
    2.0
}



#[cfg(test)]
mod tests {
    #[test]
    fn xor_single_key() {
        use hex;
        let input = "Sono un testo per testare il funzionamento";
        let bytes = super::xor_single_key(input.to_string(), 10);
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
        let table = super::get_frequency_table();
        let character: u8 = 97u8;

        let freq = table.get(character.make_ascii_uppercase());
        println!("{}", freq);
    }
}
