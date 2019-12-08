use std::cmp::min;
use std::thread;
const NUM_THREADS: u64 = 4;
pub fn is_prime(num: u64) -> bool {
    is_prime_internal(num, false)
}

fn is_prime_internal(num: u64, no_threads: bool) -> bool {
    if num == 1 || num % 2 == 0 && num != 2 || num % 3 == 0 && num != 3 {
        return false;
    } else if num == 5 || num == 7 {
        return true;
    }
    let max = integer_sqrt(num) / 6 + 1;
    if no_threads || num < 65536 {
        return check_prime(1, max, num);
    } else {
        return threaded_is_prime(num, max);
    }
}

fn threaded_is_prime(num: u64, max: u64) -> bool {
    let step = max / (NUM_THREADS) + 1;
    let mut children = vec![];
    for p in 1..=NUM_THREADS {
        let min_thread = step * (p - 1) + 1;
        let max_thread = min(p * step, max);
        let num_thread = num;
        children.push(thread::spawn(move || {
            check_prime(min_thread, max_thread, num_thread)
        }));
    }
    children.into_iter().all(|child| child.join().unwrap())
}

fn check_prime(min: u64, max: u64, num: u64) -> bool {
    for i in min..=max {
        if num % (6 * i - 1) == 0 || num % (6 * i + 1) == 0 {
            return false;
        }
    }
    true
}

// Implement https://en.wikipedia.org/wiki/Integer_square_root#Using_bitwise_operations
fn integer_sqrt(num: u64) -> u64 {
    if num < 2 {
        return num;
    } else {
        let s = integer_sqrt(num >> 2) << 1;
        let l = s + 1;
        if l * l <= num {
            return l;
        } else {
            return s;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate rand;
    use rand::Rng;
    fn simple_prime_check(num: u64) -> bool {
        !(2..num).any(|n| num % n == 0)
    }
    #[test]
    fn correct_primality_simple_check() {
        for n in 2..u64::pow(2, 16) {
            assert_eq!(simple_prime_check(n), is_prime(n), "{}", n)
        }
    }
    #[test]
    fn correct_primality_threaded_check() {
        let mut rng = rand::thread_rng();
        for _ in 1..u32::pow(2, 16) {
            let n: u32 = rng.gen();
            assert_eq!(
                is_prime_internal(n as u64, true),
                is_prime(n as u64),
                "{}",
                n
            )
        }
    }
}
