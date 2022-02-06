// Digital root is the recursive sum of all the digits in a number.

// Given n, take the sum of the digits of n. If that value has more than one digit, continue reducing in this way until a single-digit number is produced. The input will be a non-negative integer.

// Examples
//     16  -->  1 + 6 = 7
//    942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
// 132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
// 493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2

fn sum_digits(mut n: i64, base: i64) -> i64 {
    let mut sum = 0i64;
    while n > 0 {
        sum = sum + (n % base);
        n = n / base;
    }
    return sum;
}

// Returns tuple of (additive-persistence, digital-root)
pub fn digital_root(mut num: i64) -> i64 {
    let base = 10;
    while num >= base {
        num = sum_digits(num, base);
    }
    return num;
}
