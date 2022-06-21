use std::fmt;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

// Using <T: ToString> also works, but I use Display to be consistent
// with 'luhn-from' exercise.
impl<T: fmt::Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        const RADIX: u32 = 10;
        let mut mul = false;
        let mut sum: u32 = 0;
        let mut count: u32 = 0;
        for ch in self.to_string().chars().rev() {
            if ch.is_ascii_digit() {
                let mut x = ch.to_digit(RADIX).unwrap();
                if mul {
                    x *= 2;
                }
                if x > 9 {
                    x -= 9;
                }
                sum += x;
                count += 1;
                mul = !mul;
            } else if !ch.is_whitespace() {
                return false;
            }
        }
        count > 1 && sum % 10 == 0
    }
}
