pub fn rotate(input: &str, key: i8) -> String {
    const NUM_LETTERS: i8 = 26;
    let k = (if key < 0 { NUM_LETTERS } else { 0 } + (key % NUM_LETTERS)) as u8;

    input
        .chars()
        .map(|ch| {
            let x = (ch as u8) + k;
            let n = NUM_LETTERS as u8;
            if (ch.is_ascii_uppercase() && x > b'Z') || (ch.is_ascii_lowercase() && x > b'z') {
                (x - n) as char
            } else if ch.is_ascii_alphabetic() {
                x as char
            } else {
                ch
            }
        })
        .collect()
}
