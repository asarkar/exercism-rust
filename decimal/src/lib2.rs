use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::One;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
// https://jrsinclair.com/articles/2020/sick-of-the-jokes-write-your-own-arbitrary-precision-javascript-math-library/
// https://www.philipdaniels.com/blog/2019/rust-equality-and-ordering/
#[derive(PartialEq)]
pub struct Decimal {
    numerator: BigInt,
    denominator: BigInt,
}

impl fmt::Debug for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator != One::one() {
            write!(
                f,
                "{}/{}",
                self.numerator.to_str_radix(10),
                self.denominator.to_str_radix(10)
            )
        } else {
            write!(f, "{}", self.numerator.to_str_radix(10))
        }
    }
}

impl PartialOrd for Decimal {
    fn lt(&self, other: &Decimal) -> bool {
        Decimal::mul(self, other, true, false) < Decimal::mul(self, other, false, true)
    }
    fn le(&self, other: &Decimal) -> bool {
        self <= other
    }
    fn gt(&self, other: &Decimal) -> bool {
        !self.lt(other) && self != other
    }
    fn ge(&self, other: &Decimal) -> bool {
        self >= other
    }
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        let ord = if self < other {
            Ordering::Less
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Equal
        };
        Some(ord)
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
        Decimal::simplify(
            Decimal::mul(&self, &other, true, false)
                .checked_add(&Decimal::mul(&self, &other, false, true))
                .unwrap(),
            Decimal::mul(&self, &other, false, false),
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
            Decimal::mul(&self, &other, true, true),
            Decimal::mul(&self, &other, false, false),
        )
    }
}

impl Decimal {
    // We convert the ratio to a fraction, where the numerator
    // is the input string without the decimal, and the
    // denominator is some power of 10. The sign is
    // stored with the numerator.
    // Example: 0.2 is parsed into 2/10, which is further
    // simplified to 1/5.
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (num, pos, negative) = Decimal::parse_input(input);

        if num.is_empty() {
            return None;
        }
        let sign = if negative { Sign::Minus } else { Sign::Plus };
        let denom = if pos >= 0 {
            BigInt::from_radix_be(Sign::Plus, &[1_u8, 0_u8], 10)
                .unwrap()
                .pow((input.len() as u32) - (pos as u32))
        } else {
            One::one()
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

    // Reduce the fraction to its lowest terms by dividing the
    // numerator and denominator by the GCD.
    fn simplify(n: BigInt, d: BigInt) -> Decimal {
        let g = BigInt::from_biguint(Sign::Plus, n.magnitude().gcd(d.magnitude()));

        Decimal {
            numerator: n.checked_div(&g).unwrap(),
            denominator: d.checked_div(&g).unwrap(),
        }
    }

    // Multiply a combination of numerator and denominator from
    // two Decimals. n1 and n2 indicate numerators.
    fn mul(&self, other: &Decimal, n1: bool, n2: bool) -> BigInt {
        let (x, y) = if n1 && n2 {
            (&self.numerator, &other.numerator)
        } else if n1 && !n2 {
            (&self.numerator, &other.denominator)
        } else if !n1 && n2 {
            (&self.denominator, &other.numerator)
        } else {
            (&self.denominator, &other.denominator)
        };
        x.checked_mul(y)
            .unwrap_or_else(|| panic!("Couldn't multiply {} with {}", x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(s: &str, n: Option<u32>, d: Option<u32>) -> (String, String) {
        let expected = if n.is_some() {
            let sign = if s.starts_with('-') { "-" } else { "" };
            if d.is_some() {
                format!("{}{}/{}", sign, n.unwrap(), d.unwrap())
            } else {
                format!("{}{}", sign, n.unwrap())
            }
        } else {
            "".to_string()
        };
        let decimal = Decimal::try_from(s);
        let actual = decimal
            .map(|d| format!("{:?}", d))
            .unwrap_or_else(|| "".to_string());
        (expected, actual)
    }

    #[test]
    fn test_parse_positive() {
        for (s, n, d) in [
            ("0.011", Some(11), Some(1000)),
            ("1", Some(1), None),
            (".2", Some(1), Some(5)),
            ("+2", Some(2), None),
            ("+0.2", Some(1), Some(5)),
        ] {
            let (expected, actual) = helper(s, n, d);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_parse_negative() {
        for (s, n, d) in [("-0.1", Some(1), Some(10)), ("-10", Some(10), None)] {
            let (expected, actual) = helper(s, n, d);
            assert_eq!(expected, actual);
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
            assert_eq!(expected, actual);
        }
    }
}
