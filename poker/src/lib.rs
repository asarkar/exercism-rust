use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

// When (PartialOrd) derived on enums, variants are ordered by their discriminants.
// By default, the discriminant is smallest for variants at the top, and largest
// for variants at the bottom.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerHand {
    HighCard(Vec<u8>),
    OnePair(Vec<u8>),
    TwoPair(Vec<u8>),
    ThreeOfAKind(Vec<u8>),
    Straight(Vec<u8>),
    Flush(Vec<u8>),
    FullHouse(Vec<u8>),
    FourOfAKind(Vec<u8>),
    StraightFlush(Vec<u8>),
}

impl PokerHand {
    fn new(s: &str) -> PokerHand {
        let ranks = PokerHand::ranks(s);

        // Sort ranks in desc order by the length of their groups, and for equal-length groups,
        // break tie using the key (rank).
        // A rank group consists of the suites for the same rank.
        let mut r: Vec<(u8, u8)> = ranks.iter().map(|(k, v)| (v.len() as u8, *k)).collect();
        r.sort_unstable_by_key(|x| Reverse(*x));

        let rank_grp_len: Vec<u8> = r.iter().map(|(n, _)| *n).collect();
        let r: Vec<u8> = r.iter().map(|(_, k)| *k).collect();
        // Collect all the unique suites
        let suites: HashSet<u8> = ranks.values().flat_map(|v| v.iter()).cloned().collect();

        match (suites.len(), r.len()) {
            // Five different ranks
            (x, 5) => {
                let five_high = r == [14, 5, 4, 3, 2];
                // Check if the ranks are sequential
                let seq = five_high || r.iter().all(|k| (r[4]..(r[4] + 5)).contains(k));
                // Match number of suites
                match x {
                    1 if seq => PokerHand::StraightFlush(r),
                    1 => PokerHand::Flush(r),
                    // There's a test that says a 5-high straight is the lowest-scoring straight
                    _ if seq && x > 1 && five_high => PokerHand::Straight(vec![0; 5]),
                    _ if seq && x > 1 => PokerHand::Straight(r),
                    _ => PokerHand::HighCard(r),
                }
            }
            // Two different ranks
            (_, 2) => match rank_grp_len[..] {
                [4, 1] => PokerHand::FourOfAKind(r),
                [3, 2] => PokerHand::FullHouse(r),
                _ => unreachable!(),
            },
            // Three different ranks
            (_, 3) => match rank_grp_len[..] {
                [3, 1, 1] => PokerHand::ThreeOfAKind(r),
                [2, 2, 1] => PokerHand::TwoPair(r),
                _ => unreachable!(),
            },
            // Four different ranks
            (_, 4) => PokerHand::OnePair(r),
            _ => PokerHand::HighCard(r),
        }
    }

    // Return a set of suites and a map of rank -> suites
    fn ranks(s: &str) -> HashMap<u8, HashSet<u8>> {
        s.split_ascii_whitespace()
            .map(|w| {
                // Some ranks are double-digit
                let (rank, suite) = w.split_at(w.len() - 1);
                let suite = suite.as_bytes()[0];
                let rank = match rank.as_bytes() {
                    [b'A'] => 14,
                    [b'K'] => 13,
                    [b'Q'] => 12,
                    [b'J'] => 11,
                    // 2..=10
                    _ => rank.parse::<u8>().unwrap(),
                };
                (suite, rank)
            })
            .fold(HashMap::new(), |mut ranks, (suite, rank)| {
                ranks.entry(rank).or_insert_with(HashSet::new).insert(suite);
                ranks
            })
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut poker_hands: Vec<(PokerHand, usize)> = hands
        .iter()
        .enumerate()
        .map(|(i, s)| (PokerHand::new(s), i))
        .collect();

    // Sort in desc order
    poker_hands.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    poker_hands
        .iter()
        .filter(|(h, _)| *h == poker_hands[0].0)
        .map(|(_, i)| hands[*i])
        .collect()
}
