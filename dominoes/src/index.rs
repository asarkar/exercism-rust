use std::collections::{BTreeSet, HashMap};

pub struct LRIndex<'a> {
    idx: HashMap<u8, BTreeSet<usize>>,
    dominoes: &'a [(u8, u8)],
}

// Indicates which side was matched
#[derive(Debug, Clone, PartialEq)]
pub enum MatchSide {
    L,
    R,
}

// Lifetime in structs: https://stackoverflow.com/a/27590535/839733

/*
 * A general solution for a 2D search is to build a KD tree, or a range tree,
 * but we don't need to make queries for nearest neighbors or things like that.
 * We just want to find exact matches for tuples that have either a left
 * or a right the same as the side of the given tuple.
 *
 * Example: Given (2, 1) and side 'R', we will find all tuples that have
 * an one in it.
 *
 * We do this by using a HashMap of left and right values as the keys,
 * and the corresponding indices as the values. According to the usual
 * backtracking paradigm, we need to acquire and release an index, so
 * we provide methods to do so.
 *
 * The value is an ordered set so that if we take it out and put it
 * back again, we don't get it back for the same loop.
 */
impl<'a> LRIndex<'a> {
    pub fn new(dominoes: &'a [(u8, u8)]) -> Self {
        let mut idx: HashMap<u8, BTreeSet<usize>> = HashMap::new();

        for (i, (l, r)) in dominoes.iter().enumerate() {
            idx.entry(*l).or_default().insert(i);
            idx.entry(*r).or_default().insert(i);
        }
        Self { idx, dominoes }
    }

    fn is_used(&self, i: usize) -> bool {
        assert!(
            (0..self.dominoes.len()).contains(&i),
            "Index out of bounds: {i}"
        );
        !self.idx[&self.dominoes[i].0].contains(&i)
    }

    pub fn acquire(&mut self, i: usize) {
        assert!(!self.is_used(i), "{i} is already in use");
        self.a_r(i, |e| {
            e.remove(&i);
        });
    }

    pub fn release(&mut self, i: usize) {
        assert!(self.is_used(i), "{i} is not used");
        self.a_r(i, |e| {
            e.insert(i);
        });
    }

    fn a_r<F>(&mut self, i: usize, mut f: F)
    where
        F: FnMut(&mut BTreeSet<usize>),
    {
        let (l, r) = self.dominoes[i];
        self.idx.entry(l).and_modify(|e| {
            f(e);
        });
        self.idx.entry(r).and_modify(|e| {
            f(e);
        });
    }

    // 1. Should return all indices that have either left
    //    or right matching the the given domino side.
    // 2. Should not contain the given dominoes[i].
    // 3. Should not contain duplicate indices.
    // 4. Should contain identical dominoes with unique indices.
    pub fn candidates(&self, i: usize, side: MatchSide) -> ((u8, u8), Vec<(usize, MatchSide)>) {
        assert!(
            (0..self.dominoes.len()).contains(&i),
            "Index out of bounds: {i}"
        );
        let (left, right) = self.dominoes[i];
        let k = if side == MatchSide::R { right } else { left };
        let c = self.idx[&k]
            .iter()
            .filter(|x| **x != i)
            .map(|x| {
                let other = self.dominoes[*x];
                let side = if k == other.0 {
                    MatchSide::L
                } else {
                    MatchSide::R
                };
                (*x, side)
            })
            .collect();
        ((left, right), c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOMINOES: [(u8, u8); 4] = [(2, 1), (2, 1), (2, 3), (3, 1)];

    #[test]
    fn test_create_index() {
        let idx = LRIndex::new(&DOMINOES);

        for i in 0..DOMINOES.len() {
            assert!(!idx.is_used(i));
        }
    }

    #[test]
    fn test_acquire_and_release() {
        let mut idx = LRIndex::new(&DOMINOES);

        idx.acquire(0);

        assert!(idx.is_used(0));
        assert!(!idx.is_used(1));
        assert!(!idx.is_used(2));

        idx.release(0);

        for i in 0..DOMINOES.len() {
            assert!(!idx.is_used(i));
        }
    }

    #[test]
    fn test_candidates() {
        let idx = LRIndex::new(&DOMINOES);
        let data = [
            (
                (0, MatchSide::R),
                vec![(1, MatchSide::R), (3, MatchSide::R)],
            ),
            (
                (0, MatchSide::L),
                vec![(1, MatchSide::L), (2, MatchSide::L)],
            ),
            (
                (1, MatchSide::R),
                vec![(0, MatchSide::R), (3, MatchSide::R)],
            ),
            (
                (1, MatchSide::L),
                vec![(0, MatchSide::L), (2, MatchSide::L)],
            ),
            ((2, MatchSide::R), vec![(3, MatchSide::L)]),
            (
                (2, MatchSide::L),
                vec![(0, MatchSide::L), (1, MatchSide::L)],
            ),
            (
                (3, MatchSide::R),
                vec![(0, MatchSide::R), (1, MatchSide::R)],
            ),
            ((3, MatchSide::L), vec![(2, MatchSide::R)]),
        ];

        for ((i, side), expected) in data {
            let (_, mut actual) = idx.candidates(i, side.clone());
            actual.sort_unstable_by_key(|k| k.0);
            assert_eq!(
                expected, actual,
                "Unexpected candidates for index: {}, matching on: {:?}",
                i, side
            );
        }
    }
}
