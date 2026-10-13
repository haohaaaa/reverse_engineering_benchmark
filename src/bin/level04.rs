use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(windows)]
fn get_expected_date() -> String {
    #[repr(C)]
    struct SystemTimeWin {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        millis: u16,
    }
    unsafe extern "system" {
        fn GetLocalTime(st: *mut SystemTimeWin);
    }
    unsafe {
        let mut st = std::mem::zeroed();
        GetLocalTime(&mut st);
        format!("{:04}-{:02}-{:02}", st.year, st.month, st.day)
    }
}

#[cfg(not(windows))]
fn get_expected_date() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let z = (secs / 86400) as i64 + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1020 + doe / 1461 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u16;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u16;
    let y = (if m <= 2 { y + 1 } else { y }) as u16;
    format!("{y:04}-{m:02}-{d:02}")
}

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    let expected = get_expected_date();
    if input == expected {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        println!("today unlocked: {epoch}");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
