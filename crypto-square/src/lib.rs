pub fn encrypt(input: &str) -> String {
    let s = normalize(input);
    if s.is_empty() {
        return "".to_string();
    }
    let r = (s.len() as f32).sqrt() as usize;
    let c = ((s.len() as f32) / (r as f32)).ceil() as usize;
    // Don't forget to reserve for whitespaces
    // If r = c, we have r groups, and need r - 1 whitespaces.
    // Else, we have c groups, and need c - 1 whitespaces.
    // But since c = r + 1 when c > r, we need r whitespaces.
    let num_spaces = r - ((r == c) as usize);
    let mut buffer = vec![' '; r * c + num_spaces];
    for (start, chunk) in s.chunks(c).enumerate() {
        for (i, ch) in chunk.iter().enumerate() {
            // There are r columns in the output.
            // Thus, offset between two characters
            // of a word is r + 1.
            buffer[start + i * (r + 1)] = *ch;
        }
    }
    buffer.iter().collect()
}

fn normalize(plain: &str) -> Vec<char> {
    plain
        .chars()
        .map(|ch| ch.to_ascii_lowercase())
        .filter(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
        .collect()
}
