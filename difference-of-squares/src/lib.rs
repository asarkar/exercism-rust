pub fn square_of_sum(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    sum * sum
}

// https://helloacm.com/the-difference-between-sum-of-squares-and-square-of-the-sum/
pub fn sum_of_squares(n: u32) -> u32 {
    (2 * n + 1) * (n + 1) * n / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
