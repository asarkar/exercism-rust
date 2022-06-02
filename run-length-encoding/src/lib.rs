pub fn encode(source: &str) -> String {
    let mut count = 0;
    let mut prev = '\0';
    let mut encoded = String::new();

    for ch in source.chars() {
        if count == 0 || (ch == prev) {
            count += 1;
        } else {
            append(&mut encoded, count, prev);
            count = 1;
        }
        prev = ch;
    }
    append(&mut encoded, count, prev);
    encoded
}

fn append(encoded: &mut String, count: u32, prev: char) {
    if count > 1 {
        encoded.push_str(&count.to_string());
    }
    if prev != '\0' {
        encoded.push(prev);
    }
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count = String::new();

    for ch in source.chars() {
        if ch.is_ascii_digit() {
            count.push(ch);
        } else {
            let n = count.parse::<u32>().unwrap_or(1);
            for _ in 0..n {
                decoded.push(ch);
            }
            count.clear();
        }
    }
    decoded
}
