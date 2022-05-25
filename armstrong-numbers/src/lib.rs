pub fn is_armstrong_number(num: u32) -> bool {
    let mut x = num;
    let n = ((num as f32).log10()).ceil() as u32;

    // move keyword takes ownership of x, so it
    // can't anymore be used outside of the closure.
    std::iter::from_fn(move || {
        if x > 0 {
            let digit = x % 10;
            x /= 10;
            Some(u32::pow(digit, n))
        } else {
            None
        }
    })
    .sum::<u32>()
        == num
}
