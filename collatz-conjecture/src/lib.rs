pub fn collatz(n: u64) -> Option<u64> {
    const MAX: u64 = (u64::MAX - 1) / 3;
    let mut x = n;
    let mut count = 0;

    while x > 1 {
        if x % 2 == 0 {
            x /= 2;
        } else if x <= MAX {
            x = 3 * x + 1;
        } else {
            break;
        }
        count += 1;
    }
    Some(count).filter(|_| x == 1)
}
