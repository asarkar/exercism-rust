use std::collections::{hash_map::HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut freq_map: HashMap<String, u32> = HashMap::new();
    let mut indices: Vec<String> = Vec::new();

    word.chars().for_each(|ch| {
        let c = to_lowercase_if_not_already(ch);
        *freq_map.entry(c.clone()).or_default() += 1;
        indices.push(c);
    });

    possible_anagrams
        .iter()
        .filter(|candidate| {
            if word.len() != candidate.len() {
                false
            } else {
                let (ana, same) = is_anagram(&freq_map, &indices, candidate);
                !same && ana
            }
        })
        .copied()
        .collect()
}

fn to_lowercase_if_not_already(ch: char) -> String {
    if ch.is_uppercase() {
        ch.to_lowercase().to_string()
    } else {
        ch.to_string()
    }
}

fn is_anagram(
    freq_map: &HashMap<String, u32>,
    indices: &[String],
    candidate: &str,
) -> (bool, bool) {
    let mut f_map: HashMap<String, u32> = HashMap::new();
    let mut score = freq_map.len();
    let mut same = true;

    for (i, ch) in candidate.chars().enumerate() {
        let c = to_lowercase_if_not_already(ch);
        if !freq_map.contains_key(&c) || f_map.get(&c) == freq_map.get(&c) {
            return (false, false);
        }
        *f_map.entry(c.clone()).or_default() += 1;
        if f_map[&c] == freq_map[&c] {
            score -= 1;
        }
        same &= c == indices[i];
    }
    (score == 0, same)
}
