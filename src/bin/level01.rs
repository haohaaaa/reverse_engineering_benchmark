use std::env;
use std::process;

fn main() {
    let input = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("wrong");
            process::exit(1);
        }
    };

    if input == "pass_level01_alpha" {
        println!("welcome to level 01");
    } else {
        println!("wrong");
        process::exit(1);
    }
}
