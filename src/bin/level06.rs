use std::env;
use std::process;

fn verify_equations(input: &str) -> bool {
    let bytes = input.as_bytes();
    if bytes.len() != 8 {
        return false;
    }

    let c: Vec<i32> = bytes.iter().map(|&b| b as i32).collect();

    // 8 个联立线性方程组（满秩非齐次方程组，存在唯一整数解）
    c[0] + c[1] == 141
        && 2 * c[1] + c[2] == 185
        && c[2] + 3 * c[3] == 320
        && c[3] - c[4] == 3
        && c[4] + c[5] == 162
        && 2 * c[5] - c[6] == 103
        && c[6] + c[7] == 125
        && c[0] - c[7] == 34
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if verify_equations(&input) {
        println!("equations satisfied: system solved");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
