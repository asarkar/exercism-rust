/*
 * This is a simple exercise that's all about clean code.
 * We iterate over each row and column, and if a cell
 * contains a space, count the mines around it. We
 * use the &str.as_bytes() method to get a &[u8] slice
 * that we can index into.
 */
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let m = minefield.len();
    if m == 0 {
        return vec![];
    }
    let n = minefield[0].len();

    (0..m)
        .map(|row| {
            (0..n)
                .map(|col| match is_mine(minefield, row, col) {
                    true => '*',
                    _ => match count_mines(minefield, row, col) {
                        0 => ' ',
                        x => (x + b'0') as char,
                    },
                })
                .collect::<String>()
        })
        .collect()
}

fn count_mines(minefield: &[&str], row: usize, col: usize) -> u8 {
    neighbors(row, col, minefield.len(), minefield[0].len())
        .iter()
        .filter(|(r, c)| is_mine(minefield, *r, *c))
        .count() as u8
}

fn neighbors(row: usize, col: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let r = row as i32;
    let c = col as i32;
    [
        (r - 1, c - 1),
        (r - 1, c),
        (r - 1, c + 1),
        (r, c - 1),
        (r, c + 1),
        (r + 1, c - 1),
        (r + 1, c),
        (r + 1, c + 1),
    ]
    .iter()
    .filter_map(|&(r, c)| {
        if (0 <= r && r < m as i32) && (0 <= c && c < n as i32) {
            Some((r as usize, c as usize))
        } else {
            None
        }
    })
    .collect()
}

fn is_mine(minefield: &[&str], row: usize, col: usize) -> bool {
    minefield[row].as_bytes()[col] == b'*'
}
