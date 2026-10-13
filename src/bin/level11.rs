use std::env;
use std::process;

// 10x10 迷宫地图：0 为可通行路径，1 为不可逾越障碍墙壁
const MAZE: [[u8; 10]; 10] = [
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
    [1, 1, 1, 0, 1, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
];

fn simulate_maze(input: &str) -> bool {
    let mut r: i32 = 0;
    let mut c: i32 = 0;

    if input.is_empty() {
        return false;
    }

    for b in input.bytes() {
        match b {
            b'w' => r -= 1,
            b's' => r += 1,
            b'a' => c -= 1,
            b'd' => c += 1,
            _ => return false,
        }

        if r < 0 || r >= 10 || c < 0 || c >= 10 {
            return false;
        }

        if MAZE[r as usize][c as usize] != 0 {
            return false;
        }
    }

    r == 9 && c == 9
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if simulate_maze(&input) {
        println!("maze navigated: exit reached");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
