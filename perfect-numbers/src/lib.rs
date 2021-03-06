use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

/*
 * It can be shown that the sum of the divisors of a number
 * can be expressed as the product of the sum of the powers
 * of the prime factors of the number.
 *
 * Example:
 * n = 18, sum of divisors = 1 + 2 + 3 + 6 + 9 + 18
 * = 2^0 x 3^0 + 2^1 x 3^0 + 2^0 x 3^1 +
 *   2^1 x 361 + 2^0 x 3^2 + 2^1 x 3^2
 * = (2^0 + 2^1) x (3^0 + 3^1 + 3^2)
 * = (1 + p1) x (1 + p2 + p2^2), where p1=2, and p2=3
 *
 * So, the task reduces to finding all the prime factors
 * and the product of the sum of their powers.
 *
 * Furthermore, the highest power of a factor is the
 * number of times it divides the "remaining" number.
 * "Remaining" means the result after progressively
 * dividing the original number with the smaller prime
 * factors.
 *
 * In the example above, the highest power of 2 is 1.
 * The highest power of 3 is 2.
 */
pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    if num <= 2 {
        return Some(Classification::Deficient);
    }
    let sqrt = (num as f64).sqrt() as u64;
    let mut sum = 1_u64;
    let mut n = num;

    for i in 2..=sqrt {
        let mut curr_term = 1;
        let mut curr_sum = 1;

        while n % i == 0 {
            n /= i;
            curr_term *= i;
            curr_sum += curr_term;
        }
        sum *= curr_sum;
    }
    /*
     * This condition is to handle the case when remaining
     * n is a prime number greater than 2.
     *
     * For example, when original n = 6, sqrt = 2, and
     * remaining n = 3.
     */
    if n > 2 {
        sum *= 1 + n;
    }
    // Exclude the number for the given problem
    sum -= num;

    match sum.cmp(&num) {
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Greater => Some(Classification::Abundant),
        _ => Some(Classification::Perfect),
    }
}
