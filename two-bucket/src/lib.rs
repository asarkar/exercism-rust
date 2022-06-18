use std::cmp;
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl BucketStats {
    fn new(x: u8, y: u8, goal: u8, moves: u8) -> Self {
        if x == goal {
            BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: y,
            }
        } else {
            BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: x,
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Move {
    FillOne,
    FillTwo,
    EmptyOne,
    EmptyTwo,
    TxrFromOneToTwo,
    TxrFromTwoToOne,
}

/// Solve the bucket problem
// BFS
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut explored = HashSet::new();
    let mut frontier = VecDeque::new();
    let start_state = if start_bucket == &Bucket::One {
        (capacity_1, 0, Move::FillOne)
    } else {
        (0, capacity_2, Move::FillTwo)
    };
    // Opposite of start state is not allowed
    let forbidden_state = if start_bucket == &Bucket::One {
        (0, capacity_2)
    } else {
        (capacity_1, 0)
    };
    let start = vec![start_state];
    frontier.push_back(start);

    while let Some(path) = frontier.pop_front() {
        // Expand from the last state of the path
        let (x, y, _) = path[path.len() - 1];
        if x == goal || y == goal {
            return Some(BucketStats::new(x, y, goal, path.len() as u8));
        }
        for state in successors(x, y, capacity_1, capacity_2) {
            let s = (state.0, state.1);
            if explored.contains(&s) || s == forbidden_state {
                continue;
            }
            explored.insert(s);
            let mut path = path.to_vec();
            path.push(state.clone());
            frontier.push_back(path);
        }
    }
    None
}

fn successors(x: u8, y: u8, capacity_1: u8, capacity_2: u8) -> Vec<(u8, u8, Move)> {
    let mut states = Vec::new();

    if x > 0 {
        states.push((0, y, Move::EmptyOne));
    }
    if x < capacity_1 {
        states.push((capacity_1, y, Move::FillOne));
        if y > 0 {
            let vol = cmp::min(capacity_1 - x, y);
            states.push((x + vol, y - vol, Move::TxrFromTwoToOne));
        }
    }

    if y > 0 {
        states.push((x, 0, Move::EmptyTwo));
    }
    if y < capacity_2 {
        states.push((x, capacity_2, Move::FillTwo));
        if x > 0 {
            let vol = cmp::min(capacity_2 - y, x);
            states.push((x - vol, y + vol, Move::TxrFromOneToTwo));
        }
    }
    states
}
