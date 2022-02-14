extern crate base64;
use std::u8;

pub fn hex_to_base64(hex: String) -> String {
    // from https://stackoverflow.com/questions/26185485/how-to-convert-hexadecimal-values-to-base64-in-rust
    // Make vector of bytes from octets
    let mut bytes = Vec::new();
    for i in 0..(hex.len()/2) {
        let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    };

    base64::encode(&bytes) // now convert from Vec<u8> to b64-encoded String
} 
