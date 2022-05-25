use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Less if kmp(second_list, first_list) => Comparison::Sublist,
        Ordering::Greater if kmp(first_list, second_list) => Comparison::Superlist,
        Ordering::Equal if kmp(second_list, first_list) => Comparison::Equal,
        _ => Comparison::Unequal,
    }
}

/*
We use the KMP algorithm. First, we build an array 'lps', where lps[i] contains the
length of the longest proper prefix that matches the proper suffix of the same length
in an array ending at smaller[i].

For example:
smaller = [1, 1, 2, 1, 1, 2, 1, 1, 1]
lps[0] = 0
smaller[1] = 1, which is also a prefix &smaller[0..1]. Thus, lps[1] = 1.
smaller[2] = 2, and there are no prefixes that end in 2. lps[2] = 0.
...
lps = [0, 1, 0, 1, 2, 3, 4, 5, 2]
Note that lps[7] = 5 because &smaller[..4] = &smaller[3..8].

KMP algorithm then searches for the 'smaller' array in 'bigger' using
identical logic as used to build the 'lps' array.

Time complexity: If length of 'smaller' is 'n', building the 'lps' array
takes O(n) time. The algorithm then traverses both arrays, and never
backtracks on the 'larger' array.
If length of 'larger' is 'm', overall time complexity is O(m + n).

https://www.youtube.com/watch?v=GTJr8OvyEVQ
*/
fn kmp<T: PartialEq>(larger: &[T], smaller: &[T]) -> bool {
    let lps = lps(smaller);
    let mut i = 0;
    let mut j = 0;

    while i < larger.len() && j < smaller.len() {
        if larger[i] == smaller[j] {
            i += 1;
            j += 1;
        } else if j > 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }
    j == smaller.len()
}

fn lps<T: PartialEq>(smaller: &[T]) -> Vec<usize> {
    let mut lps: Vec<usize> = vec![0; smaller.len()];
    let mut i = 1;
    let mut j = 0;
    while i < smaller.len() {
        if smaller[i] == smaller[j] {
            lps[i] = j + 1;
            i += 1;
            j += 1;
        } else if j > 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }
    lps
}
