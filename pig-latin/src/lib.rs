use regex::Regex;

pub enum Translator {
    StartsWithVowel,
    StartsWithConsonantFollowedByQu,
    StartsWithConsonantFollowedByY,
    StartsWithConsonant,
}

impl Translator {
    fn matches(&self, m: Option<regex::Match>, chars: &[char]) -> bool {
        if let Some(mat) = m {
            match self {
                Translator::StartsWithVowel => mat.start() == 0,
                Translator::StartsWithConsonantFollowedByQu => {
                    chars[mat.start() - 1] == 'q' && chars[mat.start()] == 'u'
                }
                Translator::StartsWithConsonantFollowedByY => {
                    chars.len() > 2 && chars[mat.start() - 1] == 'y' && mat.start() >= 2
                }
                // Contains vowel sound
                _ => true,
            }
        } else {
            match self {
                Translator::StartsWithConsonantFollowedByY => {
                    chars.len() == 2 && chars[chars.len() - 1] == 'y'
                }
                // Contains no vowel sound
                _ => false,
            }
        }
    }
    fn translate(&self, m: Option<regex::Match>, s: String) -> String {
        let (left, right) = if let Some(mat) = m {
            match self {
                Translator::StartsWithVowel => (&s[..], ""),
                Translator::StartsWithConsonantFollowedByQu => s.split_at(mat.start() + 1),
                Translator::StartsWithConsonantFollowedByY => s.split_at(mat.start() - 1),
                // Contains vowel sound
                _ => s.split_at(mat.start()),
            }
        } else {
            match self {
                Translator::StartsWithConsonantFollowedByY => s.split_at(1),
                // Contains no vowel sound
                _ => (&s[..], ""),
            }
        };
        format!("{}{}ay", right, left)
    }
}

pub fn translate(input: &str) -> String {
    let vowel_re = Regex::new(r"[aeiou]|xr|yt").unwrap();
    let translators = vec![
        Translator::StartsWithVowel,
        Translator::StartsWithConsonantFollowedByQu,
        Translator::StartsWithConsonantFollowedByY,
        Translator::StartsWithConsonant,
    ];
    input
        .split_whitespace()
        .map(|w| {
            let x = w.trim();
            let mat = vowel_re.find(x);
            let chars: Vec<char> = x.chars().collect();
            translators
                .iter()
                .find(|t| t.matches(mat, &chars))
                .unwrap_or_else(|| panic!("{} didn't match any rule.", x))
                .translate(mat, chars.into_iter().collect())
        })
        .fold(String::new(), |mut s, w| {
            s.reserve(w.len() + 1);
            s.push_str(&w);
            s.push(' ');
            s
        })
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_my() {
        assert_eq!(translate("my"), "ymay");
    }
}
