/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .map(|ch| ch.to_ascii_lowercase())
        .filter_map(xcode)
        .enumerate()
        .fold(String::new(), |mut s, (i, ch)| {
            if i > 0 && (i % 5) == 0 {
                s.push(' ');
            }
            s.push(ch);
            s
        })
}

fn xcode(ch: char) -> Option<char> {
    if ch.is_ascii_lowercase() {
        let c = b'a' + b'z' - ch as u8;
        Some(c as char)
    } else if ch.is_ascii_digit() {
        Some(ch)
    } else {
        None
    }
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(xcode).collect()
}
