use std::convert::Into;

#[allow(clippy::needless_range_loop)]
pub fn get_diamond(c: char) -> Vec<String> {
    let n: usize = (2 * (c as u8 - b'A' + 1) - 1) as usize;
    let mut tmp = vec![vec![' '; n]; (n + 1) / 2];

    for i in 0..tmp.len() {
        let ch = (b'A' + i as u8) as char;
        tmp[i][n / 2 + i] = ch;
        tmp[i][n / 2 - i] = ch;
    }

    let mut top: Vec<String> = tmp
        .iter()
        .map(|row| String::from_iter(row.iter()))
        .collect();

    let mut bottom = top[..top.len() - 1].iter().rev().map(Into::into).collect();
    top.append(&mut bottom);

    top
}
