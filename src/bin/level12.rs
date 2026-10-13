use std::env;
use std::process;

// 4 轮微型 SPN (Substitution-Permutation Network) 结构
// 结合 256 阶非线性 S-Box 替换、非对齐循环移位与模加雪崩，专门对抗 Z3/SMT 符号求解器
const ROUND_KEYS: [[u32; 3]; 4] = [
    [0x243F6A88, 0x85A308D3, 0x13198A2E],
    [0x03707344, 0xA4093822, 0x299F31D0],
    [0x082EFA98, 0xEC4E6C89, 0x452821E6],
    [0x38D01377, 0xBE5466CF, 0x34E90C6C],
];

const TARGET_STATE: [u32; 3] = [0x2DC99108, 0x041F8846, 0xAF74AC4F];

#[inline(always)]
fn sbox(b: u8) -> u8 {
    let i = b as u32;
    ((i * 137 + 109) ^ (i >> 3) ^ (i << 5)) as u8
}

fn apply_sbox(w: u32) -> u32 {
    let b0 = sbox((w & 0xFF) as u8) as u32;
    let b1 = sbox(((w >> 8) & 0xFF) as u8) as u32;
    let b2 = sbox(((w >> 16) & 0xFF) as u8) as u32;
    let b3 = sbox(((w >> 24) & 0xFF) as u8) as u32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

fn verify_spn(input: &str) -> bool {
    let bytes = input.as_bytes();
    if bytes.len() != 12 {
        return false;
    }

    let mut w0 = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let mut w1 = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let mut w2 = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    for r in 0..4 {
        let b0 = apply_sbox(w0);
        let b1 = apply_sbox(w1);
        let b2 = apply_sbox(w2);

        w0 = b0.rotate_left(7) ^ b1 ^ ROUND_KEYS[r][0];
        w1 = b1.rotate_left(13) ^ b2 ^ ROUND_KEYS[r][1];
        w2 = b2.rotate_left(19) ^ b0 ^ ROUND_KEYS[r][2];

        w0 = w0.wrapping_add(w1);
        w1 = w1.wrapping_add(w2);
        w2 = w2.wrapping_add(w0);
    }

    [w0, w1, w2] == TARGET_STATE
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if verify_spn(&input) {
        println!("non-linear avalanche: verified");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
