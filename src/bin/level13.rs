use std::env;
use std::process;

// 数论不透明谓词库 (Number-theoretic Opaque Predicates)
// 对任何整数 x: x*(x+1) 永远是偶数 (恒真)
#[inline(never)]
fn opaque_even(x: u64) -> bool {
    (x.wrapping_mul(x.wrapping_add(1))) % 2 == 0
}

// 对任何整数 y: y^2 mod 4 只能是 0 或 1，永远不可能是 3 (恒假)
#[inline(never)]
fn opaque_mod4_impossible(y: u64) -> bool {
    (y.wrapping_mul(y)) % 4 == 3
}

// 对任何整数 z: z^3 - z 必能被 6 整除 (恒真)
#[inline(never)]
fn opaque_div6(z: u64) -> bool {
    (z.wrapping_mul(z).wrapping_mul(z).wrapping_sub(z)) % 6 == 0
}

fn verify_opaque(input: &str) -> bool {
    let bytes = input.as_bytes();
    let len = bytes.len();

    // 假分支陷阱 1
    if opaque_mod4_impossible(len as u64) {
        // 死代码炸弹：伪造的假校验与死循环
        let mut fake_accum = 0u64;
        for &b in bytes {
            fake_accum = fake_accum.wrapping_mul(31).wrapping_add(b as u64);
        }
        if fake_accum == 0xDEADBEEF {
            return true;
        }
        return false;
    }

    if len != 26 {
        return false;
    }

    let seed = bytes[0] as u64;

    // 恒真谓词保护第一段
    if !opaque_even(seed) {
        return false; // 不可能触发
    }

    // 校验第 1 段: "opaque_"
    if &bytes[0..7] != b"opaque_" {
        return false;
    }

    // 假分支陷阱 2
    if opaque_mod4_impossible(seed + 13) {
        println!("fake success branch");
        return true;
    }

    // 恒真谓词保护第二段
    if !opaque_div6(bytes[7] as u64) {
        return false; // 不可能触发
    }

    // 校验第 2 段: "math_invariant_"
    if &bytes[7..22] != b"math_invariant_" {
        return false;
    }

    // 假分支陷阱 3
    if opaque_mod4_impossible(seed.wrapping_mul(7)) {
        return false;
    }

    // 校验第 3 段: "pass"
    if &bytes[22..26] != b"pass" {
        return false;
    }

    opaque_even((bytes[25] as u64).wrapping_add(100))
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if verify_opaque(&input) {
        println!("de-flattening: branch verified");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
