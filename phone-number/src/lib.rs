use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    let phone_re = Regex::new(r"(?:[2-9][0-9]{2}){2}(?:[0-9]{4})").unwrap();
    let s: Vec<u8> = user_number
        .bytes()
        .filter(|ch| ch.is_ascii_digit())
        .collect();

    if s.len() == 11 && s[0] != b'1' {
        return None;
    }
    if s.len() < 10 {
        return None;
    }

    let num = String::from_utf8_lossy(&s[s.len() - 10..]);

    Some(num)
        .filter(|x| phone_re.is_match(x))
        .map(|x| x.into_owned())
}
