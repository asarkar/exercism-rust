use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut heap = BinaryHeap::<Reverse<(u32, u32)>>::from_iter(
        factors.iter().filter(|&i| *i > 0).map(|&i| Reverse((i, i))),
    );
    let mut sum = 0;
    let mut prev = 0;

    while let Some(Reverse((x, y))) = heap.pop() {
        if (x < limit) && (x != prev) {
            prev = x;
            sum += x;
        } else if x >= limit {
            break;
        }
        if (x + y) < limit {
            heap.push(Reverse((x + y, y)));
        }
    }
    sum
}
