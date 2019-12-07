use is_prime::is_prime;
use std::env;
use std::io;
use std::num::ParseIntError;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let _ = check_str_prime(&args[1]);
    } else {
        interactive_loop();
    }
}

fn interactive_loop() {
    loop {
        println!("Input number for checking if it is a prime.");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        if check_str_prime(&num).is_err() {
            continue;
        }
    }
}

fn check_str_prime(num: &str) -> Result<u64, ParseIntError> {
    match num.trim().parse() {
        Ok(n) => {
            if is_prime(n) {
                println!("This is a prime.");
            } else {
                println!("This is not a prime.");
            };
            return Ok(n);
        }
        Err(e) => {
            println!("Input could not be parsed as a number.");
            return Err(e);
        }
    }
}
