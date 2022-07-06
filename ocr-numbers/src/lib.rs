use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

lazy_static! {
    #[allow(clippy::style)]
    static ref NUMS: HashMap<String, String> = HashMap::from_iter([
            " _ \n".to_string() +
            "| |\n" +
            "|_|\n" +
            "   ",
            "   \n".to_string() +
            "  |\n" +
            "  |\n" +
            "   ",
            " _ \n".to_string() +
            " _|\n" +
            "|_ \n" +
            "   ",
            " _ \n".to_string() +
            " _|\n" +
            " _|\n" +
            "   ",
            "   \n".to_string() +
            "|_|\n" +
            "  |\n" +
            "   ",
            " _ \n".to_string() +
            "|_ \n" +
            " _|\n" +
            "   ",
            " _ \n".to_string() +
            "|_ \n" +
            "|_|\n" +
            "   ",
            " _ \n".to_string() +
            "  |\n" +
            "  |\n" +
            "   ",
            " _ \n".to_string() +
            "|_|\n" +
            "|_|\n" +
            "   ",
            " _ \n".to_string() +
            "|_|\n" +
            " _|\n" +
            "   ",
        ]
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, i.to_string()))
    );
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut s = String::new();

    // Process 4 rows at a time
    for lines in &input.lines().chunks(4) {
        let mut num_rows = 0;
        let mut nums: Vec<String> = Vec::new();

        for line in lines {
            // Process 3 columns at a time. 'j' restarts from 0 for
            // every row.
            for (j, chunk) in line.as_bytes().chunks(3).enumerate() {
                if chunk.len() != 3 {
                    // If this chunk is short, we need to consider the
                    // length of the last chunk that had length 3.
                    // For example, if a chunk is of length 5, the last
                    // processed chunk would be of length 3, and the
                    // short chunk of length 2.
                    // If there's no last processed chunk, this chunk is
                    // all there is.
                    let num_cols = nums.last().map(|s| s.len()).unwrap_or_default() + chunk.len();
                    return Err(Error::InvalidColumnCount(num_cols));
                }
                // New number
                if nums.len() <= j {
                    nums.push(std::str::from_utf8(chunk).unwrap().to_string());
                } else {
                    // Continue the jth number
                    nums[j].push('\n');
                    nums[j].push_str(std::str::from_utf8(chunk).unwrap());
                }
            }
            num_rows += 1;
        }

        if num_rows % 4 != 0 {
            return Err(Error::InvalidRowCount(num_rows));
        }
        let unknown = "?".to_string();
        for n in &nums {
            s.push_str(NUMS.get(n).unwrap_or(&unknown));
        }
        s.push(',');
    }
    // Pop the trailing ','
    s.pop();

    Ok(s)
}
