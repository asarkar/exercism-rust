/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum: i32 = 0;
    let mut i: i32 = 10;
    for ch in isbn.chars() {
        if ch.is_ascii_digit() {
            sum += ch.to_digit(10).map(|x| (x as i32) * i).unwrap();
            i -= 1;
        } else if ch == 'X' && i == 1 {
            sum += 10;
            i -= 1;
        } else if ch != '-' {
            return false;
        }
    }
    sum % 11 == 0 && i == 0
}
