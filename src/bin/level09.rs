use sha2::{Digest, Sha256};
use std::env;
use std::process;

#[cfg(windows)]
fn get_today_str() -> String {
    #[repr(C)]
    struct SystemTimeWin {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        millis: u16,
    }
    unsafe extern "system" {
        fn GetLocalTime(st: *mut SystemTimeWin);
    }
    unsafe {
        let mut st = std::mem::zeroed();
        GetLocalTime(&mut st);
        format!("{:04}/{:02}/{:02}", st.year, st.month, st.day)
    }
}

#[cfg(not(windows))]
fn get_today_str() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let z = (secs / 86400) as i64 + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1020 + doe / 1461 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u16;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u16;
    let y = (if m <= 2 { y + 1 } else { y }) as u16;
    format!("{y:04}/{m:02}/{d:02}")
}

fn hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

// 密文字节：纯密文存储，无明文常量
const CIPHERTEXT: [u8; 32] = [
    0x15, 0x1B, 0x07, 0x5E, 0x1A, 0x1B, 0x1D, 0x0C, 0x07, 0x0E, 0x0A, 0x17, 0x11, 0x10, 0x44, 0x5E,
    0x0E, 0x1F, 0x07, 0x12, 0x11, 0x1F, 0x1A, 0x5E, 0x0B, 0x10, 0x12, 0x11, 0x1D, 0x15, 0x1B, 0x1A,
];
const MASK_KEY: u8 = 0x7E;

fn decrypt_with_key(input: &str, today: &str) -> Option<String> {
    if input.len() != today.len() {
        return None;
    }
    let diff = input
        .bytes()
        .zip(today.bytes())
        .fold(0u8, |acc, (a, b)| acc ^ (a ^ b));
    let key = MASK_KEY ^ diff;
    let decrypted: Vec<u8> = CIPHERTEXT.iter().map(|&b| b ^ key).collect();
    String::from_utf8(decrypted).ok()
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    let today = get_today_str();
    let expected_hash = hash_bytes(today.as_bytes());
    let input_hash = hash_bytes(input.as_bytes());

    // 1. 散列约束
    if expected_hash == input_hash {
        // 2. 强绑定密钥解密载荷
        if let Some(msg) = decrypt_with_key(&input, &today) {
            println!("{msg}");
            return;
        }
    }

    println!("wrong");
    process::exit(1);
}
