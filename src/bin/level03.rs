use sha2::{Digest, Sha256};
use std::env;
use std::process;

// SHA-256("hash_level03_vault")
const EXPECTED_HASH: [u8; 32] = [
    0xE0, 0x22, 0x3D, 0x99, 0x37, 0x41, 0x78, 0x12, 0x4F, 0x16, 0xC4, 0xB2, 0x33, 0x72, 0x3B, 0xA6,
    0xD7, 0x5D, 0xAB, 0x58, 0x0D, 0x1C, 0xB1, 0x53, 0x97, 0x39, 0x6E, 0xEC, 0xDC, 0x8A, 0x91, 0x5A,
];

fn hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    let input_hash = hash_bytes(input.as_bytes());
    if input_hash == EXPECTED_HASH {
        let hex_str: String = input_hash.iter().map(|b| format!("{b:02x}")).collect();
        println!("hash verified: {}", &hex_str[..12]);
    } else {
        println!("wrong");
        process::exit(1);
    }
}
