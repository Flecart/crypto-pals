mod lib;

fn main() {
    test_xor_hex();
}

#[allow(dead_code)]
fn test_hex_to_base64() {
    use lib::hex_to_base64;

    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", hex_to_base64(input.to_string()).unwrap());
}

#[allow(dead_code)]
fn test_xor_hex() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";

    let test = "746865206b696420646f6e277420706c6179";

    let ouput = lib::xor_hex(a.to_string(), b.to_string()).unwrap();
    println!("{}", ouput);
    assert!(test.eq(&ouput));
}