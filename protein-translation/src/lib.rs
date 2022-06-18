use std::collections::HashMap;
use std::str;

pub struct CodonsInfo<'a>(HashMap<&'a str, &'a str>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.get_acid(codon.as_bytes())
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut res = Vec::<&str>::new();
        for codon in rna.as_bytes().chunks(3) {
            if let Some(acid) = self.get_acid(codon) {
                if acid == "stop codon" {
                    break;
                } else {
                    res.push(acid);
                }
            } else {
                return None;
            }
        }
        Some(res)
    }

    fn get_acid(&self, codon: &[u8]) -> Option<&'a str> {
        match str::from_utf8(codon) {
            Ok(c) => self.0.get(c).copied(),
            _ => None,
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo(HashMap::from_iter(pairs.iter().map(|(x, y)| (*x, *y))))
}
