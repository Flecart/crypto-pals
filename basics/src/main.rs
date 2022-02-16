pub mod lib;
extern crate hex;


fn main() {
    let text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex::decode(text).unwrap();
    let key = lib::frequency_attack(&bytes);

    let plaintext_bytes = lib::xor_single_key(&bytes, key);
    let plaintext = std::str::from_utf8(&plaintext_bytes).unwrap();
    println!("Text: {}", plaintext.to_string());
}

