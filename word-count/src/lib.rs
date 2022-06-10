use regex::Regex;
use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let word_re = Regex::new(r"\d+|([a-zA-Z]+(?:'??[a-zA-Z]+))").unwrap();

    words
        .split_whitespace()
        .flat_map(|w| word_re.find_iter(w))
        .map(|w| w.as_str().to_lowercase())
        .fold(HashMap::new(), |mut freq, w| {
            *freq.entry(w).or_default() += 1;
            freq
        })
}
