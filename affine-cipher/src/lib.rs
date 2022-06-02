use std::cmp;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let encoded = plaintext
        .chars()
        .map(|ch| ch.to_ascii_lowercase())
        // Captures by immutable borrow, closure trait: Fn
        .filter_map(|ch| xcode(ch, |x| (a * x + b) % M))
        .enumerate()
        .fold(String::new(), |mut s, (i, ch)| {
            if i > 0 && (i % 5) == 0 {
                s.push(' ');
            }
            s.push(ch);
            s
        });
    Ok(encoded)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if let Some(mmi) = mmi(a, M) {
        let decoded: String = ciphertext
            .chars()
            // Captures by immutable borrow, closure trait: Fn
            .filter_map(|ch| xcode(ch, |x| ((x - b) * (mmi as i32)) % M))
            .collect();
        Ok(decoded)
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}

// https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/
fn xcode<F: Fn(i32) -> i32>(ch: char, f: F) -> Option<char> {
    if ch.is_ascii_lowercase() {
        let x = ((ch as u8) - b'a') as i32;
        let mut i = f(x);
        if i < 0 {
            i += M;
        }
        Some((i as u8 + b'a') as char)
    } else if ch.is_ascii_digit() {
        Some(ch)
    } else {
        None
    }
}

fn is_coprime(a: i32, b: i32) -> bool {
    mmi(a, b).is_some()
}

/* MMI of a mod b is a number x such that ax % b = 1.
 * We use Extended Euclidean algorithm to find the MMI
 * (https://www.youtube.com/watch?v=Gu7iKt2SZYc).
 *
 * Say a is the larger of the two numbers. Then we can write a = bx + rem. This is known
 * as Division Theorem.
 * Then, we recurse using b and rem, given rem < b.
 * If ever b (smaller) = 0, then the two numbers are not coprimes, and there is no MMI.
 * Else if b = 1, the two numbers are coprimes, and a MMI exists.
 *
 * Example: Find the MMI of 17 mod 43.
 *  43 = 17x2 + 9
 *  17 = 9x1 + 8
 *   9 = 8x1 + 1
 *
 * Since b = 1, we stop, and do "back substitution", i.e. at each step substitute b with
 * the equation above.
 *
 * 1 = 9x1 - 8x1
 *   = 9x1 - (17x1 - 9x1)
 *      = 9x1 - 17x1 + 9x1
 *      = -17x1 + 9x2
 *   = -17x1 + (43x1 - 17x2)x2
 *      = 43x2 -17x5
 *
 * If a is larger, 2 is the answer, else -5. In our example, a is smaller.
 * Since the answer is negative, we add it with the larger, thus 43 - 5 = 38.
 * Check (17(a) x 38(MMI)) % 43(b) = 1.
 */
fn mmi(a: i32, b: i32) -> Option<u32> {
    let smaller = cmp::min(a, b);
    let larger = cmp::max(a, b);
    if let Some((x, y)) = mmi_rec(larger, smaller) {
        let mut i = if a == smaller { y } else { x };
        if i < 0 {
            i += larger;
        }
        assert_eq!(
            1,
            (a * i) % b,
            "failed check (a->{} * {}<-mmi) % {}<-b == 1",
            a,
            i,
            b
        );
        Some(i as u32)
    } else {
        None
    }
}

fn mmi_rec(larger: i32, smaller: i32) -> Option<(i32, i32)> {
    if smaller == 0 {
        return None;
    }
    let (x, y) = (larger / smaller, larger % smaller);
    if y == 1 {
        return Some((y, -x));
    }
    if let Some((a, b)) = mmi_rec(smaller, y) {
        Some((b, -b * x + a))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mmi() {
        assert_eq!(Some(38), mmi(17, 43));
        assert_eq!(Some(6), mmi(3, 17));
        assert_eq!(Some(47), mmi(23, 120));
        assert_eq!(None, mmi(5, 10));
        assert_eq!(None, mmi(54, 24));
        assert_eq!(None, mmi(48, 18));
        assert_eq!(Some(1), mmi(3, 2));
        assert_eq!(Some(3), mmi(9, 26));
        assert_eq!(Some(7), mmi(15, 26));
        assert_eq!(Some(9), mmi(3, 26));
        assert_eq!(Some(11), mmi(19, 26));
    }
}
