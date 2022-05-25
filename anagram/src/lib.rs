use std::collections::{hash_map::HashMap, HashSet};

// There are two obvious approaches:
// 1. Sort and compare: For a list of 'm' candidates where all of them are anagrams
// and the word length is 'n', it takes O(mn logn) time to sort them all.
// 2. We can compare frequency maps, takes O(n) time.
//
// It's hard to beat O(n), but if we compare character by character, we can end the
// comparison early in two cases:
// 1. When the candidate has a character not in the word.
// 2. When the candidate has a character count greater than the word.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut freq_map: HashMap<char, u32> = HashMap::new();
    let mut indices: Vec<char> = Vec::new();

    word.chars().for_each(|ch| {
        let c = to_lowercase_if_not_already(ch);
        *freq_map.entry(c).or_default() += 1;
        indices.push(c);
    });

    possible_anagrams
        .iter()
        .filter(|candidate| {
            word.len() == candidate.len() && is_anagram(&freq_map, &indices, candidate)
        })
        .copied()
        .collect()
}

fn to_lowercase_if_not_already(ch: char) -> char {
    if ch.is_uppercase() {
        ch.to_lowercase().next().unwrap()
    } else {
        ch
    }
}

fn is_anagram(freq_map: &HashMap<char, u32>, indices: &[char], candidate: &str) -> bool {
    let mut f_map: HashMap<char, u32> = HashMap::new();
    let mut score = freq_map.len();
    let mut same = true;

    for (i, ch) in candidate.chars().enumerate() {
        let c = to_lowercase_if_not_already(ch);
        if !freq_map.contains_key(&c) || f_map.get(&c) == freq_map.get(&c) {
            return false;
        }
        *f_map.entry(c).or_default() += 1;
        if f_map[&c] == freq_map[&c] {
            score -= 1;
        }
        same &= c == indices[i];
    }
    !same && (score == 0)
}
