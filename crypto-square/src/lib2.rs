use std::iter;

pub fn encrypt(input: &str) -> String {
    let s = normalize(input);
    if s.is_empty() {
        return "".to_string();
    }
    // Calculate the column size
    let col = ((s.len() as f32).sqrt()).ceil() as usize;
    // Split normalized text into column-sized chunks.
    // If required, pad each chunk on the right with spaces.
    let chunks: Vec<Vec<u8>> = s[..]
        .chunks(col)
        .map(|c| {
            c.iter()
                .chain(iter::repeat(&b' '))
                .take(col)
                .cloned()
                .collect::<Vec<u8>>()
        })
        .collect();
    // Transpose the chunks, and join back into
    // a String separated by spaces.
    transpose(chunks)
        .into_iter()
        .map(|c| c.into_iter().map(|x| x as char).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn normalize(plain: &str) -> Vec<u8> {
    plain
        .to_lowercase()
        .as_bytes()
        .iter()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .cloned()
        .collect()
}

fn transpose(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = matrix[0].len();
    let mut t = vec![Vec::new(); n];
    for row in matrix {
        for i in 0..row.len() {
            t[i].push(row[i]);
        }
    }
    t
}
