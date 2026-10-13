use std::env;
use std::process;

// 深度非线性混合布尔-算术 (Deep Non-linear MBA) 运算库
// 将简单的代数运算重写为布尔环与整数模环深度交错的高阶多项式，破坏 AST 简化与标准 SMT 求解

#[inline(never)]
fn mba_xor(x: u32, y: u32) -> u32 {
    let or_val = x | y;
    let and_val = x & y;
    or_val.wrapping_sub(and_val)
}

#[inline(never)]
fn mba_add(x: u32, y: u32) -> u32 {
    let xp = mba_xor(x, y);
    let cp = (x & y).wrapping_shl(1);
    mba_xor(xp, cp).wrapping_add((xp & cp).wrapping_mul(2))
}

#[allow(dead_code)]
#[inline(never)]
fn mba_sub(x: u32, y: u32) -> u32 {
    let xp = mba_xor(x, y);
    let bp = (!x & y).wrapping_shl(1);
    let x_ab = mba_xor(xp, bp);
    let b_ab = (!xp & bp).wrapping_shl(1);
    x_ab.wrapping_sub(b_ab)
}

#[inline(never)]
fn f1(w0: u32) -> u32 {
    mba_add(w0.rotate_left(13), 0x9e3779b9)
}

#[inline(never)]
fn f2(w0: u32, w1: u32) -> u32 {
    mba_xor(w1.rotate_right(7), mba_add(w0, 0x517cc1b7))
}

#[inline(never)]
fn f3(w0: u32, w1: u32, w2: u32) -> u32 {
    mba_add(mba_xor(w2, w0.rotate_left(5)), mba_add(w1, 0x165667b1))
}

#[inline(never)]
fn g2(z3: u32) -> u32 {
    mba_add(z3.rotate_left(11), 0x27d4eb2f)
}

#[inline(never)]
fn g1(z2: u32, z3: u32) -> u32 {
    mba_xor(z2.rotate_right(9), mba_add(z3, 0x64319cad))
}

#[inline(never)]
fn g0(z1: u32, z2: u32, z3: u32) -> u32 {
    mba_add(mba_xor(z1, z3.rotate_left(3)), mba_add(z2, 0x82a1b945))
}

// 目标校验值 (由 W 经过两轮三角双射变换后生成)
const TARGET_Z: [u32; 4] = [0x923e7711, 0x12f920e2, 0xbac78fa9, 0x9cf15d6a];

fn verify_mba(input: &[u8]) -> bool {
    if input.len() != 16 {
        return false;
    }

    let w0 = u32::from_le_bytes(input[0..4].try_into().unwrap());
    let w1 = u32::from_le_bytes(input[4..8].try_into().unwrap());
    let w2 = u32::from_le_bytes(input[8..12].try_into().unwrap());
    let w3 = u32::from_le_bytes(input[12..16].try_into().unwrap());

    // 第一阶段：上三角双射变换
    let y0 = w0;
    let y1 = mba_add(w1, f1(y0));
    let y2 = mba_add(w2, f2(y0, y1));
    let y3 = mba_add(w3, f3(y0, y1, y2));

    // 第二阶段：下三角双射变换
    let z3 = y3;
    let z2 = mba_add(y2, g2(z3));
    let z1 = mba_add(y1, g1(z2, z3));
    let z0 = mba_add(y0, g0(z1, z2, z3));

    [z0, z1, z2, z3] == TARGET_Z
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if verify_mba(input.as_bytes()) {
        println!("mba bijective system: recovered successfully");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
