use std::cmp;
use std::cmp::Reverse;

#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        match self.0.len() {
            0 => None,
            n => self.0.get(n - 1).cloned(),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.personal_top_three().get(0).cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.0.to_vec();
        scores.sort_by_key(|i| Reverse(*i));
        scores[..cmp::min(3, self.0.len())].to_vec()
    }
}
