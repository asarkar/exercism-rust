use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

pub fn find<T: Ord + Debug, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let arr = array.as_ref();
    if arr.is_empty() {
        return None;
    }
    helper(arr, key, 0, arr.len() - 1)
}

fn helper<T: Ord + Debug>(array: &[T], key: T, lo: usize, hi: usize) -> Option<usize> {
    if lo == hi {
        return Some(lo).filter(|&x| array[x] == key);
    }
    let mid = lo + (hi - lo) / 2;
    // println!("{:?}, key={:?}, lo={}, hi={}, mid={}", array, key, lo, hi, mid);
    match array[mid].cmp(&key) {
        Ordering::Equal => Some(mid),
        Ordering::Less => helper(array, key, mid + 1, hi),
        Ordering::Greater if mid > 0 => helper(array, key, lo, mid - 1),
        Ordering::Greater => None,
    }
}
