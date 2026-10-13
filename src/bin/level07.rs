use std::env;
use std::process;

fn verify_fsm(input: &str) -> bool {
    let bytes = input.as_bytes();
    if bytes.len() != 22 {
        return false;
    }

    // 控制流平坦化（Control Flow Flattening）：由状态机变量分发基本块，破坏顺序控制流
    let mut state: u32 = 0x10;
    let mut idx = 0;
    let valid = true;

    while state != 0xFF && state != 0xEE {
        match state {
            0x10 => {
                if bytes[0] == b'f' && bytes[1] == b's' {
                    idx = 2;
                    state = 0x45;
                } else {
                    state = 0xEE;
                }
            }
            0x45 => {
                if bytes[idx] == b'm' && bytes[idx + 1] == b'_' {
                    idx += 2;
                    state = 0x28;
                } else {
                    state = 0xEE;
                }
            }
            0x28 => {
                if bytes[idx] == b's' && bytes[idx + 1] == b't' && bytes[idx + 2] == b'a' {
                    idx += 3;
                    state = 0x89;
                } else {
                    state = 0xEE;
                }
            }
            0x89 => {
                if bytes[idx] == b't' && bytes[idx + 1] == b'e' && bytes[idx + 2] == b'_' {
                    idx += 3;
                    state = 0x33;
                } else {
                    state = 0xEE;
                }
            }
            0x33 => {
                if bytes[idx] == b'm' && bytes[idx + 1] == b'a' && bytes[idx + 2] == b'c' {
                    idx += 3;
                    state = 0x77;
                } else {
                    state = 0xEE;
                }
            }
            0x77 => {
                if bytes[idx] == b'h' && bytes[idx + 1] == b'i' && bytes[idx + 2] == b'n' {
                    idx += 3;
                    state = 0x51;
                } else {
                    state = 0xEE;
                }
            }
            0x51 => {
                if bytes[idx] == b'e' && bytes[idx + 1] == b'_' {
                    idx += 2;
                    state = 0x92;
                } else {
                    state = 0xEE;
                }
            }
            0x92 => {
                if bytes[idx] == b'p'
                    && bytes[idx + 1] == b'a'
                    && bytes[idx + 2] == b's'
                    && bytes[idx + 3] == b's'
                {
                    state = 0xFF; // 终态成功
                } else {
                    state = 0xEE;
                }
            }
            _ => {
                state = 0xEE; // 失败
            }
        }
    }

    state == 0xFF && valid
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if verify_fsm(&input) {
        println!("fsm state passed: valid");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
