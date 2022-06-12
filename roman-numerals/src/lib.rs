use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

/*
* 1. Find the floor of x in the table, i.e. the highest decimal
*    value 'v' that is less than or equal to 'x'.
* 2. Add the corresponding string to the answer, and subtract
     'v' from 'x'.
* 3. Repeat until x = 0.
*/
const NUM_TO_ROMAN: [(u32, &str); 13] = [
    (1, "I"),
    (4, "IV"),
    (5, "V"),
    (9, "IX"),
    (10, "X"),
    (40, "XL"),
    (50, "L"),
    (90, "XC"),
    (100, "C"),
    (400, "CD"),
    (500, "D"),
    (900, "CM"),
    (1000, "M"),
];

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut x = num;
        let mut roman = String::new();

        while x > 0 {
            let (k, s) = match NUM_TO_ROMAN.binary_search_by_key(&x, |&(a, _)| a) {
                Ok(i) => NUM_TO_ROMAN[i],
                Err(i) => NUM_TO_ROMAN[i - 1],
            };
            roman.push_str(s);
            x -= k;
        }
        Roman(roman)
    }
}
