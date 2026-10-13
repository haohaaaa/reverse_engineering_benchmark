use std::env;
use std::process;

// 预先混淆的密码字节与输出，二进制中无明文常量
const EXPECTED_BYTES: [u8; 18] = [
    0x3E, 0x25, 0x28, 0x39, 0x03, 0x30, 0x39, 0x2A, 0x39, 0x30, 0x6C, 0x6E, 0x03, 0x2F, 0x34, 0x35,
    0x3A, 0x28,
];
const OUTPUT_BYTES: [u8; 15] = [
    0x2F, 0x28, 0x3D, 0x3B, 0x39, 0x7C, 0x6C, 0x6E, 0x66, 0x7C, 0x3F, 0x30, 0x39, 0x3D, 0x2E,
];
const XOR_KEY: u8 = 0x5C;

fn verify_password(input: &str) -> bool {
    let bytes = input.as_bytes();
    if bytes.len() != EXPECTED_BYTES.len() {
        return false;
    }
    for i in 0..bytes.len() {
        if bytes[i] ^ XOR_KEY != EXPECTED_BYTES[i] {
            return false;
        }
    }
    true
}

fn get_success_message() -> String {
    let raw: Vec<u8> = OUTPUT_BYTES.iter().map(|&b| b ^ XOR_KEY).collect();
    String::from_utf8(raw).unwrap_or_default()
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if verify_password(&input) {
        println!("{}", get_success_message());
    } else {
        println!("wrong");
        process::exit(1);
    }
}
