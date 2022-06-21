use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(k, v)| v.iter().map(|ch| (ch.to_ascii_lowercase(), *k)))
        .collect()
}
