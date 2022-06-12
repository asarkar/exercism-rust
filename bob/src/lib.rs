pub fn reply(message: &str) -> &str {
    let mut num_upper = 0;
    let mut num_lower = 0;
    let mut m = String::new();

    for ch in message.chars() {
        num_upper += ch.is_ascii_uppercase() as u32;
        num_lower += ch.is_ascii_lowercase() as u32;

        if ch.is_ascii_alphanumeric() || ch == '?' {
            m.push(ch);
        }
    }
    let question = m.ends_with('?');
    let address = m.is_empty();
    let yelling = num_lower == 0 && num_upper > 0;

    if address {
        "Fine. Be that way!"
    } else if yelling && question {
        "Calm down, I know what I'm doing!"
    } else if yelling {
        "Whoa, chill out!"
    } else if question {
        "Sure."
    } else {
        "Whatever."
    }
}
