// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

// LeetCode 383: Ransom Note.
pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mag_word_freq: HashMap<&str, u32> = HashMap::new();
    for w in magazine {
        *mag_word_freq.entry(w).or_default() += 1;
    }
    for w in note {
        if mag_word_freq
            .get(w)
            .map(|i| i.to_owned())
            .unwrap_or_default()
            == 0
        {
            return false;
        }
        *mag_word_freq.entry(w).or_default() -= 1;
    }
    true
}
