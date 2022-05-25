/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    const RADIX: u32 = 10;
    let mut mul = false;
    let mut sum: u32 = 0;
    let mut count: u32 = 0;
    for ch in code.chars().rev() {
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
