use std::env;
use std::process;

// 可逆双工海绵虚拟机 (Reversible Sponge VM)
// 击溃局部前缀贪心（Prefix Domino）爆破：
// 无任何中间局部验证点，指令序列中穿插非线性完全可逆海绵置换层 (Reversible Feistel Sponge)，
// 确保每个输入比特在整段执行中实现全局全域扩散 (Full Global Diffusion)。
// 逆向者必须将整条指令流自底向上整体求逆，无法采用单步贪心前缀破解。

const OP_XOR_IMM: u8 = 1;
const OP_ADD_IMM: u8 = 2;
const OP_ROTL_IMM: u8 = 3;
const OP_MIX_ADD: u8 = 4;
const OP_MIX_XOR: u8 = 5;
const OP_SPONGE: u8 = 6;

#[derive(Clone, Copy)]
struct Instruction {
    op: u8,
    dst: usize,
    src: usize,
    imm: u32,
}

const PROGRAM: [Instruction; 18] = [
    Instruction { op: OP_XOR_IMM, dst: 0, src: 0, imm: 0x5a5a5a5a },
    Instruction { op: OP_ADD_IMM, dst: 1, src: 0, imm: 0x12345678 },
    Instruction { op: OP_ROTL_IMM, dst: 2, src: 0, imm: 7 },
    Instruction { op: OP_MIX_ADD, dst: 3, src: 0, imm: 0 },
    Instruction { op: OP_SPONGE, dst: 0, src: 0, imm: 0x9e3779b9 },
    Instruction { op: OP_MIX_XOR, dst: 1, src: 2, imm: 0 },
    Instruction { op: OP_ADD_IMM, dst: 2, src: 0, imm: 0x76543210 },
    Instruction { op: OP_ROTL_IMM, dst: 3, src: 0, imm: 11 },
    Instruction { op: OP_MIX_ADD, dst: 0, src: 3, imm: 0 },
    Instruction { op: OP_SPONGE, dst: 0, src: 0, imm: 0x85ebca6b },
    Instruction { op: OP_XOR_IMM, dst: 3, src: 0, imm: 0xa5a5a5a5 },
    Instruction { op: OP_ADD_IMM, dst: 0, src: 0, imm: 0xfedcba98 },
    Instruction { op: OP_ROTL_IMM, dst: 1, src: 0, imm: 13 },
    Instruction { op: OP_MIX_XOR, dst: 2, src: 0, imm: 0 },
    Instruction { op: OP_SPONGE, dst: 0, src: 0, imm: 0xc2b2ae35 },
    Instruction { op: OP_MIX_ADD, dst: 1, src: 3, imm: 0 },
    Instruction { op: OP_ROTL_IMM, dst: 0, src: 0, imm: 5 },
    Instruction { op: OP_XOR_IMM, dst: 2, src: 0, imm: 0x33445566 },
];

const TARGET_REGS: [u32; 4] = [0xc49bd597, 0x56014b1f, 0x112cf812, 0x9146e638];

fn sponge_round_fwd(state: &mut [u32; 4], round_const: u32) {
    state[0] ^= state[1].rotate_left(5) ^ round_const;
    state[1] = state[1].wrapping_add(state[2].rotate_left(9));
    state[2] ^= state[3].rotate_left(13);
    state[3] = state[3].wrapping_add(state[0].rotate_left(17));
}

fn execute_vm(regs: &mut [u32; 4]) {
    for inst in PROGRAM {
        match inst.op {
            OP_XOR_IMM => {
                regs[inst.dst] ^= inst.imm;
            }
            OP_ADD_IMM => {
                regs[inst.dst] = regs[inst.dst].wrapping_add(inst.imm);
            }
            OP_ROTL_IMM => {
                regs[inst.dst] = regs[inst.dst].rotate_left(inst.imm);
            }
            OP_MIX_ADD => {
                regs[inst.dst] = regs[inst.dst].wrapping_add(regs[inst.src]);
            }
            OP_MIX_XOR => {
                regs[inst.dst] ^= regs[inst.src];
            }
            OP_SPONGE => {
                sponge_round_fwd(regs, inst.imm);
            }
            _ => {}
        }
    }
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    let bytes = input.as_bytes();
    if bytes.len() != 16 {
        println!("wrong");
        process::exit(1);
    }

    let mut regs = [
        u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
        u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
    ];

    execute_vm(&mut regs);

    if regs == TARGET_REGS {
        println!("reversible sponge vm: state inverted to source");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
