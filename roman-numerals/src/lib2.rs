use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

/*
 * 1. Since all the 2-digit Roman numerals are multiples of 10,
 *    we create a number 'y' by zeroing out all the digits of
 *    the given number except the first.
 *
 *    For example, 977 gives 900, 107 gives 100, 27 gives 20, and so on.
 *
 * 2. Then we try to find a single numeral or a combination of two
 *    numerals, representing a subtraction, that is closest to 'y'.
 *
 *    For example, y=100 matches 'C', whereas y=900 is given by
 *    'CM' (=1000-1000).
 *
 *    We do this by finding the ceiling of 'y', and if a match
 *    exists, like for y=100, we reduce the original number by 'y',
 *    and iterate.
 *
 *    Else, we look for a combination given by (ceiling of y - y),
 *    like for y=900; if we find such a combination, we reduce the
 *    original number by 'y', and iterate.
 *
 * 3. If step 2 didn't yield a match, like for y=300, we find the
 *    floor of 'y', which is 100 in this example. We reduce the
 *    original number by the floor of 'y', and iterate.
 *
 * Time complexity: In the worst case, the number of iterations
 * is equal to the number of characters in the output. At each
 * iteration, we do no more than two range searches, each of which
 * takes O(log n) time, where n = 7, thus practically constant time.
 */
impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let num_to_roman = BTreeMap::from([
            (1, 'I'),
            (5, 'V'),
            (10, 'X'),
            (50, 'L'),
            (100, 'C'),
            (500, 'D'),
            (1000, 'M'),
        ]);
        let mut x = num;
        let mut roman = String::new();

        while x > 0 {
            let num_digits = (x as f32).log10().ceil() as u32;
            let y = if num_digits > 1 {
                let factor = 10_u32.pow(num_digits - 1);
                (x / factor) * factor
            } else {
                x
            };

            if let Some((ceiling_y, c1)) = num_to_roman.range(y..).next() {
                if *ceiling_y == y {
                    roman.push(*c1);
                    x -= y;
                    continue;
                }
                if let Some(c2) = num_to_roman.get(&(ceiling_y - y)) {
                    roman.push(*c2);
                    roman.push(*c1);
                    x -= y;
                    continue;
                }
            }

            if let Some((floor_y, c)) = num_to_roman.range(..y).next_back() {
                roman.push(*c);
                x -= floor_y;
            }
        }
        Roman(roman)
    }
}
