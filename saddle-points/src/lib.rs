use std::cmp::Ordering;
use std::collections::HashSet;

#[allow(clippy::needless_range_loop)]
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() {
        return vec![];
    }
    // One row may have more than one maximum value only if there are duplicates
    // Example: Given [[1, 1, 1]], row_max = [{1, 1, 1}]
    let mut row_max: Vec<HashSet<usize>> = vec![HashSet::new(); input.len()];
    // One col may have more than one minimum value only if there are duplicates
    // Example: Given [[1, 1, 1]], col_min = [{1}, {1}, {1}]
    let mut col_min: Vec<HashSet<usize>> = vec![HashSet::new(); input[0].len()];

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let v = input[row][col];

            let x = &mut row_max[row];
            if x.is_empty() {
                x.insert(col);
            } else {
                // Compare current value with any row max
                match x.iter().next().map(|c| input[row][*c]).unwrap().cmp(&v) {
                    // Current value greater than previous row max, discard those
                    Ordering::Less => {
                        x.clear();
                        x.insert(col);
                    }
                    // Current value equal to row max
                    Ordering::Equal => {
                        x.insert(col);
                    }
                    _ => {}
                };
            }

            let y = &mut col_min[col];
            if y.is_empty() {
                y.insert(row);
            } else {
                // Compare current value with any col min
                match y.iter().next().map(|r| input[*r][col]).unwrap().cmp(&v) {
                    // Current value smaller than previous col min, discard those
                    Ordering::Greater => {
                        y.clear();
                        y.insert(row);
                    }
                    // Current value equal to col min
                    Ordering::Equal => {
                        y.insert(row);
                    }
                    _ => {}
                };
            }
        }
    }

    row_max
        .iter()
        .enumerate()
        .flat_map(|(row, cols)| {
            // Shadow col_min with a reference to the outer variable.  The closure
            // will capture this reference instead of the col_min value above.
            let col_min = &col_min;
            // Max columns in 'row'
            cols.iter()
                // Check if the min of this column contains current row
                .filter(move |c| col_min[**c].contains(&row))
                .map(move |c| (row, *c))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let mat = vec![vec![1, 1, 1]];
        assert_eq!(
            vec![(0, 0), (0, 1), (0, 2)],
            find_sorted_saddle_points(&mat)
        );

        let mat = vec![vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
        assert_eq!(vec![(1, 0)], find_sorted_saddle_points(&mat))
    }

    fn find_sorted_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
        let mut result = find_saddle_points(input);
        result.sort_unstable();
        result
    }
}
