use std::fmt;

pub struct Luhn {
    val: String,
}

// Convert input to a String by requiring T implement Display.
impl<T> From<T> for Luhn
where
    T: fmt::Display,
{
    fn from(input: T) -> Self {
        Luhn {
            val: format!("{}", input),
        }
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        const RADIX: u32 = 10;
        let mut mul = false;
        let mut sum: u32 = 0;
        let mut count: u32 = 0;
        for ch in self.val.chars().rev() {
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
