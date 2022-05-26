use lazy_static::lazy_static;
use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    lazy_static! {
        static ref ALPHABETIC_RE: Regex = Regex::new(r"([A-Za-z']+)").unwrap();
    }

    // Split the phrase into words.
    ALPHABETIC_RE
        .captures_iter(phrase)
        .flat_map(|w| {
            let word = &w[1];
            let letters = extract_upper_case(word);
            if letters.is_empty() {
                vec![first_ch(word)]
            } else {
                letters
            }
        })
        .collect()
}

// Extracts all uppercase letters ignoring consecutive ones.
// "PNG" -> ["P"], "lower" -> [], "CamelCase" -> ['C', 'C']
fn extract_upper_case(word: &str) -> Vec<char> {
    lazy_static! {
        static ref UPPER_CASE_RE: Regex = Regex::new(r"([A-Z])").unwrap();
    }
    UPPER_CASE_RE
        .find_iter(word)
        .fold((-1, Vec::new()), |(prev_end, mut acc), x| {
            if (x.start() as i32) != prev_end {
                acc.push(first_ch(x.as_str()));
            }
            (x.end() as i32, acc)
        })
        .1
}

fn first_ch(word: &str) -> char {
    word.chars().next().unwrap().to_ascii_uppercase()
}
