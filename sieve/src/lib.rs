use std::cmp::Reverse;
use std::collections::BinaryHeap;
/*
 * https://www.khanacademy.org/computing/computer-science/cryptography/comp-number-theory/v/sieve-of-eratosthenes-prime-adventure-part-4
 *
 * Instead of crossing out all the multiples of a prime immediately,
 * we put its square on a priority queue, along with the prime, so
 * that we can generate the multiples when we need to.
 *
 * Suppose xy = n = √n√n. If x ≥ √n, then y ≤ √n and vice-versa.
 * Thus, if xy = n, then one of x or y must be less than or equal to √n.
 * This means that if n can be factored, one of the factors must be less
 * than or equal to √n.
 * Thus, putting only composites <= √n on the pq is sufficient.
 *
 * Whenever we pop an item from the queue, we also push its next multiple.
 *
 * The next smallest composite is always on the top, so we can check
 * if a number if prime or not in O(1) time.
 *
 * Here is a sample run for upper_bound=13, n=14, sqrt=3.
 * Notes:
 * - i=5 onwards, the squares aren't pushed because those are > 3.
 * - 12 is put twice on the queue, once from 9+3 and then from 10+2.
 *   Thus, the queue may have the same number more than once.
 * - The 2nd item of the tuple, the prime, isn't used for ordering,
 *   so, even though the table below shows (12, 2) coming before
 *   (12, 3), this is not guaranteed. We could, of course, include
 *   prime in the ordering key, but we don't need to.
 * +----+----------------------+--------------------+
 * | i  |        Primes        |     Composites     |
 * +----+----------------------+--------------------+
 * |  2 | [2]                  | [(4, 2)]           |
 * |  3 | [2, 3]               | [(4, 2), (9, 3)]   |
 * |  4 | [2, 3]               | [(6, 2), (9, 3)]   |
 * |  5 | [2, 3, 5]            | [(6, 2), (9, 3)]   |
 * |  6 | [2, 3, 5]            | [(8, 2), (9, 3)]   |
 * |  7 | [2, 3, 5, 7]         | [(8, 2), (9, 3)]   |
 * |  8 | [2, 3, 5, 7]         | [(9, 3), (10, 2)]  |
 * |  9 | [2, 3, 5, 7]         | [(10, 2), (12, 3)] |
 * | 10 | [2, 3, 5, 7]         | [(12, 2), (12, 3)] |
 * | 11 | [2, 3, 5, 7, 11]     | [(12, 2), (12, 3)] |
 * | 12 | [2, 3, 5, 7, 11]     | [(14, 2)]          |
 * | 13 | [2, 3, 5, 7, 11, 13] | [(14, 2)]          |
 * +----+----------------------+--------------------+
 *
 * Time complexity: The outer loop runs n-2 times, and the if-else does
 * a constant number of O(log n) operations.
 * Overall time complexity is O(n log n).
 */
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let n = upper_bound + 1;
    let sqrt = (n as f64).sqrt() as u64;
    let mut primes = Vec::<u64>::new();
    let mut composites = BinaryHeap::<Reverse<(u64, u64)>>::new();

    for i in 2..n {
        if composites.is_empty() || composites.peek().unwrap().0 .0 != i {
            let x = i * i;
            if i <= sqrt {
                composites.push(Reverse((x, i)));
            }
            primes.push(i);
        } else {
            while !composites.is_empty() && composites.peek().unwrap().0 .0 == i {
                let (x, y) = composites.pop().unwrap().0;
                if x + y <= n {
                    composites.push(Reverse((x + y, y)));
                }
            }
        }
    }
    primes
}
