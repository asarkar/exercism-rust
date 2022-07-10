pub fn abbreviate(phrase: &str) -> String {
    let s = phrase.as_bytes();

    /*
     * 1. If a character is the start of a word, take it.
     *    Words are separated by whitespaces.
     * 2. If a character is preceded by a hyphen, take it.
     * 3. If a character is upper case and not preceded by
     *    another upper case character, take it.
     */
    s.iter()
        .enumerate()
        .filter_map(|(i, b)| {
            let c = *b as char;
            let prev = if i == 0 {
                char::default()
            } else {
                s.get(i - 1).cloned().unwrap() as char
            };
            let first =
                c.is_ascii_alphabetic() && (i == 0 || prev.is_ascii_whitespace() || prev == '-');
            if first || (c.is_ascii_uppercase() && !prev.is_ascii_uppercase()) {
                Some(c.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect()
}
