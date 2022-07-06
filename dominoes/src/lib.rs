mod index;
use index::{LRIndex, MatchSide};

type Domino = (u8, u8);

/*
 * The problem can be rephrased to say that given some pairs/tuples,
 * arrange them such that the adjacent sides have the same number.
 * You might think that given [(1, 1), (2, 2)], a possible answer is
 * [(1, 2), (2, 1)], but no, because the pairs (1, 2) and (2, 1)
 * don't exist in the input. However, given [(1, 2) and (1, 2)],
 * [(1, 2), (2, 1)] is acceptable because we simply flipped the
 * tuple (2, 1).
 *
 * Thus, the problem reduces to finding tuples with either the left
 * or the right matching a given number. This is done in the module
 * index.
 */
pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let n = input.len();
    let mut dominoes = Vec::new();
    let mut idx = LRIndex::new(input);
    // Due to the symmetry in the problem, it doesn't matter which
    // domino or side we start from.
    let complete = helper(&mut idx, 0, MatchSide::R, &mut dominoes, n);

    Some(dominoes).filter(|d| complete && !d.is_empty())
}

fn helper(
    idx: &mut LRIndex,
    i: usize,
    side: MatchSide,
    dominoes: &mut Vec<Domino>,
    n: usize,
) -> bool {
    let ((left, right), candidates) = idx.candidates(i, side.clone());

    // If matched on the left side,
    // this tuple has to be flipped.
    // Example: (2, 1) and (2, 3).
    if side == MatchSide::L {
        dominoes.push((right, left));
    } else {
        dominoes.push((left, right));
    }

    // To print acquire and release order nicely with left-padding,
    // see https://stackoverflow.com/a/41821049/839733
    // https://doc.rust-lang.org/std/fmt/
    idx.acquire(i);

    for (k, side) in candidates {
        let side = match side {
            // If matched on the right side
            // this tuple has to be flipped
            // (in the next iteration),
            // Example: (2, 1) and (3, 1)
            MatchSide::R => MatchSide::L,
            _ => MatchSide::R,
        };
        if helper(idx, k, side, dominoes, n) {
            return true;
        }
    }

    idx.release(i);

    if dominoes.len() == n {
        return dominoes
            .first()
            .zip(dominoes.last())
            .filter(|(x, y)| x.0 == y.1)
            .is_some();
    }

    dominoes.pop();

    false
}
