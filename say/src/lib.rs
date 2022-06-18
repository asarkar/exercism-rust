use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::collections::HashMap;

lazy_static! {
    static ref NUM_ZEROS: BTreeMap<u64, &'static str> = BTreeMap::from_iter(
        [
            (2, "hundred"),
            (3, "thousand"),
            (6, "million"),
            (9, "billion"),
            (12, "trillion"),
            (15, "quadrillion"),
            (18, "quintillion"),
        ]
        .into_iter()
        .map(|(i, s)| (u64::pow(10, i), s))
    );
    static ref NUMS: HashMap<u64, &'static str> = HashMap::from([
        (0, ""),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety")
    ]);
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        "zero".to_string()
    } else {
        encode_n(n)
    }
}

fn encode_n(n: u64) -> String {
    if let Some(s) = NUMS.get(&n) {
        return s.to_string();
    }
    // Find the number with the greatest number of zeros
    // less than or equal to n.
    // Example: For 120, this will find 100.
    if let Some((i, s)) = NUM_ZEROS.range(..=n).next_back() {
        let (left, right) = (n / i, n % i);
        vec![encode_n(left), s.to_string(), encode_n(right)]
            .join(" ")
            .trim()
            .to_string()
    }
    // n < 100
    else {
        let (left, right) = (n / 10 * 10, n % 10);
        format!("{}-{}", NUMS[&left], NUMS[&right])
    }
}
