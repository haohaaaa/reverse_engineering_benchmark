use std::env;
use std::process;

#[cfg(windows)]
fn check_debugger_api() -> bool {
    unsafe extern "system" {
        fn IsDebuggerPresent() -> i32;
    }
    unsafe { IsDebuggerPresent() != 0 }
}

#[cfg(not(windows))]
fn check_debugger_api() -> bool {
    false
}

#[inline(always)]
fn get_cycles() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

fn verify_with_anti_debug(input: &str) -> bool {
    // 1. 静态环境反调试 API 探测
    if check_debugger_api() {
        return false;
    }

    // 2. RDTSC 动态时钟周期差探测（如果在调试器中下断点单步跟踪，时钟周期差会达到数百万）
    let start_cycles = get_cycles();

    let target = b"anti_debug_timing_gate";
    let input_bytes = input.as_bytes();
    let mut matched = input_bytes.len() == target.len();

    for i in 0..input_bytes.len().min(target.len()) {
        if input_bytes[i] != target[i] {
            matched = false;
        }
    }

    let end_cycles = get_cycles();

    // 正常执行仅需数百到数千时钟周期，若单步调试断点停顿，周期差将极其庞大（> 10,000,000）
    if end_cycles > start_cycles && (end_cycles - start_cycles) > 20_000_000 {
        return false;
    }

    matched
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if verify_with_anti_debug(&input) {
        println!("integrity pass: clear");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
