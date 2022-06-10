use itertools::Itertools;
use std::cmp;
use std::collections::{hash_map::HashMap, HashSet};
const DISCOUNTS: [u32; 6] = [0, 0, 40, 80, 160, 200];
const COST_OF_ONE: u32 = 800;

/*
 * 1. For this problem, it only matters how many different types of books are there.
 * Instead of operating on the input array, we are going to be working with a
 * frequency array. So, [1, 1, 2] becomes [0, 2, 1, 0, 0, 0]. The array is of
 * size six for convenience, so, that we can use the books directly as indices
 * without having to subtract 1.
 *
 * 2. A naive caching/memoization approach would use the frequency array directly as
 * the key, but note that [0, 1, 2, 0, 0, 0] and [0, 2, 1, 0, 0, 0] cost exactly the
 * same. Thus, we use a sorted, non-zero, frequency array as the cache key.
 * In the above example, it is [1, 2].
 *
 * 3. Lastly, note that [1, 1] (zeros removed) will produce three combinations,
 * [[1], [1], [1, 1]], of which the first two are identical. Thus, we deduplicate
 * the combinations before iterating on them.
 */
pub fn lowest_price(books: &[u32]) -> u32 {
    let mut freq = vec![0; 6];
    for b in books {
        freq[*b as usize] += 1;
    }
    helper(&mut freq, &mut HashMap::new())
}
fn helper(books: &mut Vec<u32>, memo: &mut HashMap<Vec<u32>, u32>) -> u32 {
    // books=[0, 1, 2] and books=[2, 0, 1] have the same cost
    let key = find_non_zero(books, |(_, &count)| count);
    if memo.contains_key(&key) {
        return memo[&key];
    }
    // Find the books that can be added to the basket
    let remaining = find_non_zero(books, |(b, _)| b);

    if remaining.is_empty() {
        memo.insert(key, 0);
        return 0;
    }

    let mut cost = u32::MAX;
    let mut basket = HashSet::new();
    let books_copy = books.to_vec();

    // Try adding all combinations of the remaining books to the basket
    for i in 1..=remaining.len() {
        for choice in remaining
            .iter()
            .combinations(i)
            // Dedup combinations that have the same counts.
            // For example, books=[1, 1] has three combinations [[1], [1], [1, 1]],
            // where the second combination is redundant.
            .unique_by(|c| c.iter().map(|&x| books_copy[*x]).collect::<Vec<u32>>())
        {
            for book in &choice {
                basket.insert(**book);
                books[**book] -= 1;
            }
            // Check the cost of the current basket, and start a new basket
            cost = cmp::min(cost, get_cost(basket.len()) + helper(books, memo));
            // Undo the current combination and try another
            for book in &choice {
                basket.remove(book);
                books[**book] += 1;
            }
        }
    }
    memo.insert(key, cost);
    cost
}

fn find_non_zero<T: Ord, F>(books: &[u32], f: F) -> Vec<T>
where
    F: FnMut((usize, &u32)) -> T,
{
    let mut result: Vec<T> = books
        .iter()
        .enumerate()
        .filter(|(_, &count)| count > 0)
        .map(f)
        .collect();

    result.sort();
    result
}

fn get_cost(n: usize) -> u32 {
    (COST_OF_ONE - DISCOUNTS[n]) * n as u32
}
