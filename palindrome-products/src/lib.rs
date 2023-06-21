use std::cmp::Ordering;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        Some(value)
            .filter(|v| {
                let x = format!("{:?}", v);
                let y: String = x.chars().rev().collect();
                x == y
            })
            .map(Palindrome)
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let smallest = palindrome(min, max, 1).and_then(Palindrome::new);
    let largest = palindrome(max, min, -1).and_then(Palindrome::new);

    smallest.zip(largest)
}

/*
 * Searches for a palindrome with factors in the given range.
 * Whether the palindrome is the maximum or minimum depends
 * on the given step.
 */
fn palindrome(start: u64, stop: u64, step: i64) -> Option<u64> {
    if signum((stop as i64) - (start as i64)) == -step {
        return None;
    }
    let mut palindrome: Option<u64> = None;
    let mut left = start;
    let op = if step < 0 {
        PartialOrd::ge
    } else {
        PartialOrd::le
    };

    while op(&left, &stop) {
        let mut right = start;
        /*
         * This variable determines whether the last iterations of the inner
         * loop (right) produced a product that satisfied the given condition
         * when compared with the palindrome found so far. Since the ranges are
         * monotonically increasing/decreasing, if no such product was found
         * in the last iteration, it won't be found in any future iterations
         * either, so, we can stop.
         */
        let mut should_continue = false;

        // One factor is smaller or equal to the other.
        while op(&right, &left) {
            let pdt = left * right;
            if palindrome.filter(|p| !op(&pdt, p)).is_none() {
                should_continue = true;
                let x = pdt.to_string();
                if x == x.chars().rev().collect::<String>() {
                    _ = palindrome.insert(pdt);
                }
            }

            if step < 0 {
                right -= 1;
            } else {
                right += 1;
            }
        }
        if !should_continue {
            break;
        }
        if step < 0 {
            left -= 1;
        } else {
            left += 1;
        }
    }
    palindrome
}

fn signum(n: i64) -> i64 {
    match n.cmp(&0) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_product_does_not_use_the_smallest_factor() {
        let (min, max) = palindrome_products(3215, 4000).unwrap();
        assert_eq!(10988901, min.0);
        assert_eq!(15600651, max.0);
    }
}
