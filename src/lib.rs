use std::cmp::min;
use std::thread;
static NUM_THREADS: u128 = 2;
#[inline]
pub fn is_prime(num: u128) -> bool {
    if num == 1 {
        return false;
    }
    if num % 2 == 0 && num != 2 {
        return false;
    }
    if num % 3 == 0 && num != 3 {
        return false;
    }
    let max = integer_sqrt(num) / 6 + 1;
    if num < 4196 {
        return check_prime(1, max, num);
    } else {
        return threaded_is_prime(num, max);
    }
}

#[inline]
fn threaded_is_prime(num: u128, max: u128) -> bool {
    let step = max / (NUM_THREADS - 1);
    let mut children = vec![];
    for p in 1..=NUM_THREADS {
        let min_thread = p * (step - 1) + 1;
        let max_thread = min(p * step, max);
        let num_thread = num;
        children.push(thread::spawn(move || {
            return check_prime(min_thread, max_thread, num_thread);
        }));
    }
    children.into_iter().all(|child| child.join().unwrap())
}

#[inline]
fn check_prime(min: u128, max: u128, num: u128) -> bool {
    for i in min..=max {
        if num % (6 * i - 1) == 0 || num % (6 * i + 1) == 0 {
            return false;
        }
    }
    true
}

// Implement https://en.wikipedia.org/wiki/Integer_square_root#Using_bitwise_operations
#[inline]
fn integer_sqrt(num: u128) -> u128 {
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
