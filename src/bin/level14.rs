use std::env;
use std::process;

// 级联自修改虚拟机指令集 (Cascade Self-Modifying VM)
const OP_PUSH: u8 = 0x01;
const OP_LOAD_INPUT: u8 = 0x02;
const OP_CMP: u8 = 0x03;
const OP_JNZ: u8 = 0x04;
const OP_DECRYPT: u8 = 0x05;
const OP_LEN: u8 = 0x06;
const OP_HALT_OK: u8 = 0xFE;
const OP_HALT_FAIL: u8 = 0xFF;

// 级联自解密字节码：除前 51 字节外，后续 Block 1 与 Block 2 均为密文！
// 静态反编译只能看到首个校验块；后续指令必须由上一段输入的字符在运行时动态解密！
const INITIAL_BYTECODE: [u8; 137] = [
    0x06, 0x01, 0x0F, 0x03, 0x04, 0xFF, 0x01, 0x00, 0x02, 0x01, 0x73, 0x03, 0x04, 0xFF, 0x01, 0x01,
    0x02, 0x01, 0x65, 0x03, 0x04, 0xFF, 0x01, 0x02, 0x02, 0x01, 0x6C, 0x03, 0x04, 0xFF, 0x01, 0x03,
    0x02, 0x01, 0x66, 0x03, 0x04, 0xFF, 0x01, 0x04, 0x02, 0x01, 0x5F, 0x03, 0x04, 0xFF, 0x05, 0x33,
    0x25, 0x00, 0x05, 0x72, 0x60, 0x6E, 0x67, 0x32, 0x70, 0x61, 0x93, 0x67, 0x59, 0x71, 0x64, 0x03,
    0x65, 0x5B, 0x8C, 0x64, 0x6B, 0x64, 0x5E, 0x17, 0x66, 0x68, 0x99, 0x5E, 0x7B, 0x67, 0x6D, 0x39,
    0x5C, 0x77, 0x9A, 0x69, 0x3E, 0x6E, 0x76, 0x61, 0x6C, 0x66, 0x66, 0x5E, 0x1B, 0x6C, 0x60, 0xA0,
    0x6C, 0x65, 0x66, 0x5E, 0x00, 0x6C, 0x60, 0xA0, 0x6C, 0x64, 0x66, 0x5E, 0x32, 0x6C, 0x60, 0xA0,
    0x6C, 0x63, 0x66, 0x5E, 0x06, 0x6C, 0x60, 0xA0, 0x6C, 0x62, 0x66, 0x5E, 0x08, 0x6C, 0x60, 0xA0,
    0x6C, 0x61, 0x66, 0x5E, 0x14, 0x6C, 0x60, 0xA0, 0x93,
];

struct SelfModifyingVM<'a> {
    mem: [u8; 137],
    pc: usize,
    stack: Vec<u8>,
    flag_equal: bool,
    input: &'a [u8],
}

impl<'a> SelfModifyingVM<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            mem: INITIAL_BYTECODE,
            pc: 0,
            stack: Vec::with_capacity(16),
            flag_equal: false,
            input,
        }
    }

    fn run(&mut self) -> bool {
        while self.pc < self.mem.len() {
            let opcode = self.mem[self.pc];
            self.pc += 1;

            match opcode {
                OP_PUSH => {
                    let imm = self.mem[self.pc];
                    self.pc += 1;
                    self.stack.push(imm);
                }
                OP_LOAD_INPUT => {
                    let idx = self.stack.pop().unwrap_or(0) as usize;
                    let val = self.input.get(idx).copied().unwrap_or(0);
                    self.stack.push(val);
                }
                OP_CMP => {
                    let exp = self.stack.pop().unwrap_or(0);
                    let act = self.stack.pop().unwrap_or(0);
                    self.flag_equal = act == exp;
                }
                OP_JNZ => {
                    let target = self.mem[self.pc] as usize;
                    self.pc += 1;
                    if !self.flag_equal {
                        self.pc = target;
                    }
                }
                OP_LEN => {
                    self.stack.push(self.input.len() as u8);
                }
                OP_DECRYPT => {
                    // 原位自解密下一段字节码：
                    // [start_target, len, in_start, in_len]
                    let target_offset = self.mem[self.pc] as usize;
                    let target_len = self.mem[self.pc + 1] as usize;
                    let in_start = self.mem[self.pc + 2] as usize;
                    let in_len = self.mem[self.pc + 3] as usize;
                    self.pc += 4;

                    if in_len > 0 && in_start + in_len <= self.input.len() {
                        let key = &self.input[in_start..in_start + in_len];
                        for i in 0..target_len {
                            if target_offset + i < self.mem.len() {
                                self.mem[target_offset + i] ^= key[i % in_len];
                            }
                        }
                    }
                }
                OP_HALT_OK => return true,
                OP_HALT_FAIL => return false,
                _ => return false, // 遇到未解密或解密错的非法指令直接崩溃
            }
        }
        false
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

    let mut vm = SelfModifyingVM::new(input.as_bytes());
    if vm.run() {
        println!("cascade vm: all blocks unlocked");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
