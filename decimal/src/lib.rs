use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::Zero;
use std::cmp::{Ordering, PartialOrd};
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Write as _;
use std::ops::{Add, Mul, Neg, Sub};
use std::str::FromStr;

/// Type implementing arbitrary-precision decimal arithmetic
// https://jrsinclair.com/articles/2020/sick-of-the-jokes-write-your-own-arbitrary-precision-javascript-math-library/
// https://www.philipdaniels.com/blog/2019/rust-equality-and-ordering/
#[derive(PartialEq)]
pub struct Decimal {
    numerator: BigInt,
    denominator: usize,
}

impl fmt::Debug for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/10^{}",
            self.numerator.to_str_radix(10),
            self.denominator
        )
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        // Using a u64::pow(10, ?) here may overflow
        let x = format!("1{}", "0".repeat(other.denominator));
        let y = format!("1{}", "0".repeat(self.denominator));

        BigInt::from_str(&x)
            .ok()
            .and_then(|i| self.numerator.checked_mul(&i))
            .zip(
                BigInt::from_str(&y)
                    .ok()
                    .and_then(|j| other.numerator.checked_mul(&j)),
            )
            .map(|(i, j)| i.cmp(&j))
    }
}

impl Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Decimal {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (larger, smaller) = if self.denominator > other.denominator {
            (self, other)
        } else {
            (other, self)
        };
        let factor = larger.denominator - smaller.denominator;
        let x = format!("1{}", "0".repeat(factor));

        Decimal::simplify(
            // Using a u64::pow(10, factor) here may overflow
            smaller.numerator * BigInt::from_str(&x).unwrap() + larger.numerator,
            larger.denominator,
        )
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.add(-other)
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Decimal::simplify(
            self.numerator * other.numerator,
            self.denominator + other.denominator,
        )
    }
}

impl Decimal {
    // We convert the ratio to a fraction, where the numerator
    // is the input string without the decimal and leading zeros,
    // and the denominator is the power of 10.
    // The sign is stored with the numerator.
    // Example: 0.2 is parsed into 2/1.
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (num, pos, negative) = Decimal::parse_input(input);

        if num.is_empty() {
            return None;
        }
        let sign = if negative { Sign::Minus } else { Sign::Plus };
        let denom = if pos >= 0 {
            input.len() - (pos as usize)
        } else {
            0_usize
        };
        Some(Decimal::simplify(
            BigInt::from_radix_be(sign, &num, 10).unwrap(),
            denom,
        ))
    }

    // Parse the input string, and figure out the sign,
    // and position of the decimal point, if any.
    fn parse_input(input: &str) -> (Vec<u8>, i64, bool) {
        let mut num = Vec::new();
        let mut pos = -1_i64;
        let mut negative = false;

        for (i, b) in input.as_bytes().iter().enumerate() {
            if !b.is_ascii_digit() {
                if i == 0 && (*b == b'-' || *b == b'+') {
                    negative = *b == b'-';
                } else if *b == b'.' && pos == -1 {
                    pos = (i + 1) as i64;
                } else {
                    return (vec![], pos, negative);
                }
            } else {
                num.push(*b - b'0');
            }
        }
        (num, pos, negative)
    }

    // Reduce the fraction to its lowest terms by dividing
    // repeatedly by ten.
    fn simplify(n: BigInt, d: usize) -> Decimal {
        let mut x = n;
        let mut y = d;
        let ten = BigInt::from_str("10").unwrap();

        while y > 0 && x.mod_floor(&ten) == Zero::zero() {
            x /= 10;
            y -= 1;
        }

        Decimal {
            numerator: x,
            denominator: y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(s: &str, n: Option<u32>, d: Option<u32>) -> (String, String) {
        let expected = if let Some(n) = n {
            let sign = if s.starts_with('-') { "-" } else { "" };
            let mut x = format!("{}{}", sign, n);
            if let Some(d) = d {
                let _ = write!(x, "/10^{}", d);
            }
            x
        } else {
            "".to_string()
        };
        let decimal = Decimal::try_from(s);
        let actual = decimal.map(|d| format!("{:?}", d)).unwrap_or_default();
        (expected, actual)
    }

    #[test]
    fn test_parse_positive() {
        for (s, n, d) in [
            ("0.011", Some(11), Some(3)),
            ("1", Some(1), Some(0)),
            (".2", Some(2), Some(1)),
            ("+2", Some(2), Some(0)),
            ("+0.2", Some(2), Some(1)),
            ("1.0", Some(1), Some(0)),
            ("0.0", Some(0), Some(0)),
        ] {
            let (expected, actual) = helper(s, n, d);
            assert_eq!(expected, actual, "Parsing of {} failed", s);
        }
    }

    #[test]
    fn test_parse_negative() {
        for (s, n, d) in [
            ("-0.1", Some(1), Some(1)),
            ("-10", Some(10), Some(0)),
            ("-1.99", Some(199), Some(2)),
        ] {
            let (expected, actual) = helper(s, n, d);
            assert_eq!(expected, actual, "Parsing of {} failed", s);
        }
    }

    #[test]
    fn test_parse_invalid() {
        for (s, n, d) in [
            ("0.1.", None, None),
            ("-1-0", None, None),
            ("1a2", None, None),
        ] {
            let (expected, actual) = helper(s, n, d);
            assert_eq!(expected, actual, "Parsing of {} failed", s);
        }
    }
}
