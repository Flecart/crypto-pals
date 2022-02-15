extern crate base64;
extern crate hex;
use std::u8;

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