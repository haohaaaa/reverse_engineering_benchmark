#![allow(non_camel_case_types)]
use std::env;
use std::process;

const ROUND_KEYS: [u8; 16] = [
    0x31, 0x41, 0x59, 0x26, 0x53, 0x58, 0x97, 0x93,
    0x23, 0x84, 0x62, 0x64, 0x33, 0x83, 0x27, 0x95,
];

const TARGET_CIPHER: [u8; 16] = [
    0xd7, 0xaa, 0xdc, 0x5a, 0x4c, 0xe2, 0x1c, 0x0f,
    0x94, 0x8f, 0x65, 0xd5, 0xa4, 0x64, 0x66, 0x59,
];

#[cfg(windows)]
mod win {
    use std::ffi::c_void;

    #[repr(C, align(16))]
    pub struct CONTEXT {
        pub p1_home: u64,
        pub p2_home: u64,
        pub p3_home: u64,
        pub p4_home: u64,
        pub p5_home: u64,
        pub p6_home: u64,
        pub context_flags: u32,
        pub mx_csr: u32,
        pub seg_cs: u16,
        pub seg_ds: u16,
        pub seg_es: u16,
        pub seg_fs: u16,
        pub seg_gs: u16,
        pub seg_ss: u16,
        pub eflags: u32,
        pub dr0: u64,
        pub dr1: u64,
        pub dr2: u64,
        pub dr3: u64,
        pub dr6: u64,
        pub dr7: u64,
        pub rax: u64,
        pub rcx: u64,
        pub rdx: u64,
        pub rbx: u64,
        pub rsp: u64,
        pub rbp: u64,
        pub rsi: u64,
        pub rdi: u64,
        pub r8: u64,
        pub r9: u64,
        pub r10: u64,
        pub r11: u64,
        pub r12: u64,
        pub r13: u64,
        pub r14: u64,
        pub r15: u64,
        pub rip: u64,
    }

    #[repr(C)]
    pub struct EXCEPTION_RECORD {
        pub exception_code: u32,
        pub exception_flags: u32,
        pub exception_record: *mut EXCEPTION_RECORD,
        pub exception_address: *mut c_void,
        pub number_parameters: u32,
        pub exception_information: [usize; 15],
    }

    #[repr(C)]
    pub struct EXCEPTION_POINTERS {
        pub exception_record: *mut EXCEPTION_RECORD,
        pub context_record: *mut CONTEXT,
    }

    pub type PvectoredExceptionHandler = unsafe extern "system" fn(*mut EXCEPTION_POINTERS) -> i32;

    unsafe extern "system" {
        pub fn AddVectoredExceptionHandler(first: u32, handler: PvectoredExceptionHandler) -> *mut c_void;
        pub fn RemoveVectoredExceptionHandler(handle: *mut c_void) -> u32;
    }

    pub const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
    pub const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
    pub const EXCEPTION_ILLEGAL_INSTRUCTION: u32 = 0xC000001D;
}

#[cfg(windows)]
unsafe extern "system" fn veh_dispatcher(info: *mut win::EXCEPTION_POINTERS) -> i32 {
    unsafe {
        let rec = &*(*info).exception_record;
        let ctx = &mut *(*info).context_record;

        if rec.exception_code == win::EXCEPTION_ILLEGAL_INSTRUCTION {
            let round = ctx.rax as usize;
            let ptr = ctx.r8 as *mut u8;

            if round < 16 && !ptr.is_null() {
                let state = std::slice::from_raw_parts_mut(ptr, 16);
                let shift = (round % 5) as u32 + 1;
                state[round] = (state[round] ^ ROUND_KEYS[round]).rotate_left(shift);
                let next = (round + 1) % 16;
                state[next] ^= state[round];
            }

            // 跳过 2 字节 ud2 指令 (0x0F, 0x0B)
            ctx.rip += 2;
            return win::EXCEPTION_CONTINUE_EXECUTION;
        }
    }
    win::EXCEPTION_CONTINUE_SEARCH
}

fn execute_state_machine(state: &mut [u8; 16]) {
    #[cfg(all(windows, target_arch = "x86_64"))]
    unsafe {
        let handle = win::AddVectoredExceptionHandler(1, veh_dispatcher);
        for round in 0..16u64 {
            let ptr = state.as_mut_ptr();
            core::arch::asm!(
                "mov r8, {0}",
                "mov rax, {1}",
                "ud2",
                in(reg) ptr,
                in(reg) round,
                out("rax") _,
                out("r8") _,
            );
        }
        win::RemoveVectoredExceptionHandler(handle);
    }

    #[cfg(not(all(windows, target_arch = "x86_64")))]
    {
        for round in 0..16 {
            let shift = (round % 5) as u32 + 1;
            state[round] = (state[round] ^ ROUND_KEYS[round]).rotate_left(shift);
            let next = (round + 1) % 16;
            state[next] ^= state[round];
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

    let mut state = [0u8; 16];
    state.copy_from_slice(bytes);

    execute_state_machine(&mut state);

    if state == TARGET_CIPHER {
        println!("veh kernel dispatcher: all traps resolved");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
