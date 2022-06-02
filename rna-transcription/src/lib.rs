use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Dna(Vec<char>);

#[derive(Debug, PartialEq)]
pub struct Rna(Vec<char>);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let valid_nucleotides = HashSet::from(['G', 'C', 'T', 'A']);
        let mut tmp = Dna(Vec::new());
        for (i, ch) in dna.chars().enumerate() {
            valid_nucleotides.get(&ch).ok_or(i)?;
            tmp.0.push(ch);
        }
        Ok(tmp)
    }

    pub fn into_rna(self) -> Rna {
        let mapping = HashMap::from([('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]);
        let rna: String = self.0.iter().map(|c| mapping[c]).collect();
        Rna::new(&rna).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let valid_nucleotides = HashSet::from(['G', 'C', 'U', 'A']);
        let mut tmp = Rna(Vec::new());
        for (i, ch) in rna.chars().enumerate() {
            valid_nucleotides.get(&ch).ok_or(i)?;
            tmp.0.push(ch);
        }
        Ok(tmp)
    }
}
