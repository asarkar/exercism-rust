use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

pub fn count(lines: &[&str]) -> u32 {
    let lines: Vec<&[u8]> = lines.iter().map(|l| l.as_bytes()).collect();

    // Find the corner indices, for each row and col.
    // Example, given the following rectangle, where top left = (0, 0)
    //  +-+
    //  | |
    //  +-+
    // row_corners={0: {0, 2}, 2: {0, 2}}
    // col_corners={0: {0, 2}, 2: {0, 2}}
    let (row_corners, col_corners) = corner_indices(&lines);

    row_corners
        .into_iter()
        .map(|(r, c)| {
            // Consider a corner pair in row 'r', and check if this pair
            // forms the top left and top right of a rectangle.
            c.into_iter()
                .combinations(2)
                .map(|c| {
                    let (c1, c2) = (c[0], c[1]);
                    // Get the other rows that have the same corner pair.
                    col_corners
                        .get(&c1)
                        .zip(col_corners.get(&c2))
                        // Count valid rectangles with top left = (r, c1), top right = (r, c2)
                        // and bottoms on the rows that have both c1 and c2 corners.
                        .map(|(r1, r2)| {
                            let rows: Vec<usize> =
                                r1.intersection(r2).filter(|x| **x > r).cloned().collect();
                            count_rect(&lines, r, c1, c2, rows)
                        })
                        .unwrap_or_default()
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

type CornerIndices = BTreeMap<usize, BTreeSet<usize>>;

fn corner_indices(lines: &[&[u8]]) -> (CornerIndices, CornerIndices) {
    lines
        .iter()
        .enumerate()
        .flat_map(|(r, l)| l.iter().positions(|ch| *ch == b'+').map(move |c| (r, c)))
        .fold(
            (BTreeMap::new(), BTreeMap::new()),
            |(mut rows, mut cols), (row, col)| {
                rows.entry(row).or_insert_with(BTreeSet::new).insert(col);
                cols.entry(col).or_insert_with(BTreeSet::new).insert(row);
                (rows, cols)
            },
        )
}

// Count valid rectangles with top left = (row, c1), top right = (row, c2)
// and bottoms on the rows that have both c1 and c2 corners.
fn count_rect(lines: &[&[u8]], row: usize, c1: usize, c2: usize, rows: Vec<usize>) -> u32 {
    let mut k = row;
    let mut count = 0;
    for row in rows {
        // A valid connection between two rows is made up of either a '|' or a '+'.
        let rows_connected =
            (k + 1..row).all(|r| is_valid(lines, r, c1, b'|') && is_valid(lines, r, c2, b'|'));
        // A valid connection between two cols is made up of either a '-' or a '+'
        let cols_connected =
            (c1 + 1..c2).all(|c| is_valid(lines, k, c, b'-') && is_valid(lines, row, c, b'-'));

        if rows_connected && cols_connected {
            count += 1;
        } else {
            // Early exit: If row 0 is not connected to row 1,
            // row 0 can't be connected to row 2.
            break;
        }
        // Optimization using the transitive property of connectedness:
        // If row 0 is connected to row 1, and row 1 is connected to row 2,
        // then row 0 is connected to row 2.
        // Since we already checked from row 0 to row 1, it is sufficient
        // to check from row 1 to row 2.
        k = row;
    }
    count
}

fn is_valid(lines: &[&[u8]], row: usize, col: usize, valid: u8) -> bool {
    let i: u8 = Default::default();
    let actual = lines[row].get(col).cloned().unwrap_or(i);

    actual == valid || actual == b'+'
}
