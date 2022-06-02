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

/*
 * Let palindrome = first * second
 *  = (10^n - x) * (10^n - y), where x and y are positive integers
 *  = 10^(2n) - 10^n * (x + y) + xy
 *  = 10^n * (10^n — (x + y)) + xy
 *  = 10^n * left + right, where left = 10^n — (x + y) and right = xy ---(i)
 *
 * Define z = x + y ---(ii)
 * left = 10^n — z
 * right = x(z-x) using (i) and (ii)
 *  = -x^2 + zx
 * Therefore, x^2 - zx + right = 0 ---(iii)
 *
 * Now, two n digits numbers could generate a 2n - 1 or 2n digits palindrome.
 * For example, palindrome number 'abcddcba' for n=4 can be written as:
 *  10^4*abcd + dcba
 *  = 10^4*left + right.
 *
 * Thus, we can substitute for right the reverse of left in the equation (iii).
 *
 * From (i) and (ii), left = 10^n — z > 0. Since we are given min and max,
 * min <= 10^n — z <= max
 *  => 10^n - max <= z <= 10^n - min.
 */
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let smallest = min_palindrome_pdt(min, max).and_then(Palindrome::new);
    let largest = max_palindrome_pdt(min, max).and_then(Palindrome::new);

    smallest.zip(largest)
}

/*
 * Recall that for a quadratic equation ax^2 + bx + c = 0,
 * the two solutions are given by (-b ± √(b^2 - 4ac)) / (2a),
 * which has at least one real solution if the discriminant (b^2 - 4ac) >= 0.
 *
 * Solve x^2 - zx + right = 0.
 * a = 1, b = -z, c = right.
 * Discriminant d = (b^2 - 4ac) = z^2 - 4 * right.
 * Roots: (-b ± √d) / 2a = (z ± √d) / 2
 */
fn min_palindrome_pdt(min: u64, max: u64) -> Option<u64> {
    let n = format!("{:?}", max).len();
    if n == 1 {
        return Some(1);
    }

    let hi = u64::pow(10, n as u32);
    let r = min..=max;

    for z in ((hi - max)..=(hi - min)).rev() {
        let left: u64 = hi - z;
        let right: u64 = reverse(left);

        if !r.contains(&left) || !r.contains(&right) {
            continue;
        }

        return Some(u64::pow(10, (n - 1) as u32) * left + right);
    }
    None
}

fn max_palindrome_pdt(min: u64, max: u64) -> Option<u64> {
    let n = format!("{:?}", max).len();
    if n == 1 {
        return Some(9);
    }

    let hi = u64::pow(10, n as u32);

    for z in (hi - max)..=(hi - min) {
        let left: u64 = hi - z;
        let right: u64 = reverse(left);

        let d = (z * z) as f32 - (4 * right) as f32;
        if d >= 0.0 {
            let x = f32::sqrt(d);
            let root1 = ((z as f32) + x) / 2f32;
            let root2 = ((z as f32) - x) / 2f32;
            if root1.fract() == 0.0 || root2.fract() == 0.0 {
                return Some(hi * left + right);
            }
        }
    }
    None
}

fn reverse(i: u64) -> u64 {
    format!("{:?}", i)
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}
