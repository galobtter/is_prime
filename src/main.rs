use is_prime::is_prime;
use std::io;
fn main() {
    println!("Input number for checking if it is a prime.");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: u64 = num
        .trim()
        .parse()
        .expect("Input could not be parsed as a number.");
    if is_prime(num) {
        println!("This is a prime.");
    } else {
        println!("This is not a prime.");
    };
}
