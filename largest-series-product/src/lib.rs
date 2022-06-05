use std::cmp;

#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

/*
 * To avoid repeated multiplications, we maintain a running product.
 * Whenever we exceed the length of a span, we divide the product
 * by the outgoing number, provided the outgoing number is not zero.
 * We also keep track of the index of the last zero, because as long
 * as there is a zero in the span, the product is zero. We restart
 * the product when we get past a zero.
 *
 * Example: string_digits = 576802143, span = 2.
 *
 * +---+-------+---------+
 * | i | digit | product |
 * +---+-------+---------+
 * | 0 |     5 | 5       |
 * | 1 |     7 | 5*7     |
 * | 2 |     6 | 7*6     |
 * | 3 |     8 | 6*8     |
 * | 4 |     0 | 0       |
 * | 5 |     2 | 2       |
 * | 6 |     1 | 2*1     |
 * | 7 |     4 | 1*4     |
 * | 8 |     3 | 4*3     |
 * +---+-------+---------+
 *
 * The maximum product=48 is found in i=3.
 *
 * Time complexity: O(n), where n = string_digits.len().
 */
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if span == 0 {
        return Ok(1);
    }
    let mut pdt: u64 = 1;
    let mut max_pdt: u64 = 0;
    let mut start: usize = 0;
    let mut last_index_of_zero: usize = string_digits.len();
    let digits: Vec<char> = string_digits.chars().collect();

    for i in 0..digits.len() {
        let ch = digits[i];
        if !ch.is_ascii_digit() {
            return Err(Error::InvalidDigit(ch));
        }

        let k = ch.to_digit(10).unwrap() as u64;
        if k == 0 {
            last_index_of_zero = i;
        }
        if i > 0 && last_index_of_zero == (i - 1) {
            pdt = k;
        } else {
            pdt *= k;
        }

        if (i - start + 1) == span {
            if !(start..=i).contains(&last_index_of_zero) {
                max_pdt = cmp::max(max_pdt, pdt);
                if digits[start] as u8 != b'0' {
                    pdt /= digits[start].to_digit(10).unwrap() as u64;
                }
            }
            start += 1;
        }
    }
    Ok(max_pdt)
}
