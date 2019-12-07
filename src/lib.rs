#[inline]
pub fn is_prime(num: u128) -> bool {
    if num % 2 == 0 {
        return false;
    }
    if num % 3 == 0 {
        return false;
    }
    let max = integer_sqrt(num) / 6;
    for i in 1..=max {
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
