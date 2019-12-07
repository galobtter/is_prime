use is_prime::is_prime;
use std::io;
fn main() {
    loop {
        println!("Input number for checking if it is a prime.");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        match num.trim().parse() {
            Ok(n) => {
                if is_prime(n) {
                    println!("This is a prime.");
                } else {
                    println!("This is not a prime.");
                };
            }
            Err(_) => {
                println!("Input could not be parsed as a number.");
                continue;
            }
        }
    }
}
