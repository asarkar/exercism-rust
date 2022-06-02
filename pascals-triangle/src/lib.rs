pub struct PascalsTriangle {
    n: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { n: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.n).map(nth_row).collect()
    }
}

/*
 * Identity: C(n,k+1) = C(n,k) * (n-k) / (k+1), where n starts from 0.
 * We start with C(n,0) = 1, and calculate the rest using the identity.
 * But wait...each row is mirrored around the middle element, so,
 * we only need to calculate up to the middle element. Then we
 * flip the row and append to itself.
 */
fn nth_row(n: u32) -> Vec<u32> {
    let mut row: Vec<u32> = vec![1];
    let mid = (n / 2) as usize;
    for k in 0..mid {
        let i = k as u32;
        row.push(row[k] * (n - i) / (i + 1));
    }

    let take_until = mid + (n % 2) as usize;
    let mut right: Vec<u32> = row[..take_until].iter().rev().copied().collect();
    row.append(&mut right);
    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_row() {
        assert_eq!(vec![1], nth_row(0));
        assert_eq!(vec![1, 1], nth_row(1));
        assert_eq!(vec![1, 2, 1], nth_row(2));
        assert_eq!(vec![1, 3, 3, 1], nth_row(3));
        assert_eq!(vec![1, 4, 6, 4, 1], nth_row(4));
        assert_eq!(vec![1, 5, 10, 10, 5, 1], nth_row(5));
    }
}
