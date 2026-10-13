use std::env;
use std::process;

// 15-数码 (15-Puzzle) 置换群验证器
// 状态空间高达 16! / 2 ≈ 1.04 x 10^13
// 盲目广度优先搜索 (BFS) 在深度 20 时搜索树达到 3^20 ≈ 3.5 x 10^9 节点，当场内存耗尽崩溃
// 必须通过启发式搜索 (IDA* 结合曼哈顿距离) 才能在数毫秒内解出最优 20 步解

const INITIAL_BOARD: [u8; 16] = [
    1, 2, 7, 11,
    5, 6, 8, 4,
    9, 15, 0, 3,
    13, 10, 14, 12,
];

const GOAL_BOARD: [u8; 16] = [
    1, 2, 3, 4,
    5, 6, 7, 8,
    9, 10, 11, 12,
    13, 14, 15, 0,
];

const MAX_ALLOWED_MOVES: usize = 20;

fn verify_puzzle(moves: &str) -> bool {
    let bytes = moves.as_bytes();
    if bytes.len() != MAX_ALLOWED_MOVES {
        return false;
    }

    let mut board = INITIAL_BOARD;

    for &m in bytes {
        let zero_idx = match board.iter().position(|&x| x == 0) {
            Some(idx) => idx,
            None => return false,
        };

        let row = (zero_idx / 4) as isize;
        let col = (zero_idx % 4) as isize;

        let (dr, dc) = match m {
            b'U' => (-1, 0),
            b'D' => (1, 0),
            b'L' => (0, -1),
            b'R' => (0, 1),
            _ => return false,
        };

        let new_row = row + dr;
        let new_col = col + dc;

        if new_row < 0 || new_row >= 4 || new_col < 0 || new_col >= 4 {
            return false;
        }

        let new_idx = (new_row * 4 + new_col) as usize;
        board.swap(zero_idx, new_idx);
    }

    board == GOAL_BOARD
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if verify_puzzle(&input) {
        println!("15-puzzle: permutation group solved in 20 moves");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
