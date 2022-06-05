use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Sieve of Eratosthenes
pub fn nth(n: u32) -> u32 {
    let mut prime: u32 = 0;
    let mut count = 0;
    let mut composites = BinaryHeap::<Reverse<(u64, u32)>>::new();

    for i in 2.. {
        if composites.is_empty() || composites.peek().unwrap().0 .0 != i {
            prime = i as u32;
            if count == n {
                break;
            }
            count += 1;
            let x = i * i;
            composites.push(Reverse((x, i as u32)));
        } else {
            while !composites.is_empty() && composites.peek().unwrap().0 .0 == i {
                let (x, y) = composites.pop().unwrap().0;
                composites.push(Reverse((x + y as u64, y)));
            }
        }
    }
    prime
}
