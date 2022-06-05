pub fn series(digits: &str, len: usize) -> Vec<String> {
    // This is nonsense but there is a test case for this
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    let mut result = Vec::new();
    let mut start = 0;

    for i in 0..digits.len() {
        if (i - start + 1) == len {
            result.push(digits[start..=i].to_string());
            start += 1;
        }
    }
    result
}
