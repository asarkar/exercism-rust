use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna).and_then(|x| x.get(&nucleotide).ok_or('X').copied())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = vec!['A', 'C', 'G', 'T']
        .into_iter()
        .map(|x| (x, 0))
        .collect();

    for x in dna.chars() {
        if !counts.contains_key(&x) {
            return Err('X');
        }
        counts.entry(x).and_modify(|e| *e += 1);
    }
    Ok(counts)
}
