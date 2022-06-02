#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().fold(Vec::new(), |mut acc, v| {
        acc.append(&mut encode(*v));
        acc
    })
}

// Encoding is basically converting to base 128, with the sign bit
// set for all but the last byte.
fn encode(i: u32) -> Vec<u8> {
    if i == 0 {
        return vec![0];
    }
    const BASE: u32 = 128;
    let mut x = i;
    let mut y: Vec<u8> = std::iter::from_fn(move || {
        if x > 0 {
            let digit = x % BASE;
            x /= BASE;
            Some(digit)
        } else {
            None
        }
    })
    .enumerate()
    .map(|(i, d)| {
        let x = if i == 0 { d & !BASE } else { d | BASE };
        x as u8
    })
    .collect();

    y.reverse();
    y
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    const BASE: u8 = 128;
    let mut start = 0;
    bytes
        .iter()
        .enumerate()
        .try_fold(Vec::new(), |mut acc, (i, &b)| {
            if (b & BASE) == 0 {
                let x = decode(&bytes[start..=i])?;
                start = i + 1;
                acc.push(x);
            }
            Ok(acc)
        })
        .and_then(|x| {
            if x.is_empty() {
                Err(Error::IncompleteNumber)
            } else {
                Ok(x)
            }
        })
}

fn decode(bytes: &[u8]) -> Result<u32, Error> {
    const BASE: u8 = 128;
    bytes
        .iter()
        .rev()
        .enumerate()
        .try_fold(0_u64, |acc, (i, &b)| {
            if i == 0 && (b & BASE) > 0 {
                Err(Error::IncompleteNumber)
            } else {
                let x = acc + u64::pow(BASE as u64, i as u32) * ((b & !BASE) as u64);
                if x > u32::MAX.into() {
                    Err(Error::Overflow)
                } else {
                    Ok(x)
                }
            }
        })
        .map(|v| v as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(vec![0], encode(0));
        assert_eq!(vec![129, 9], encode(137));
        assert_eq!(vec![4], encode(4));
        assert_eq!(vec![129, 0], encode(128));
        assert_eq!(vec![127], encode(127));
        assert_eq!(vec![129, 128, 128, 0], encode(2097152));
    }

    #[test]
    fn test_decode() {
        for i in &[0, 137, 4, 128, 127, 2097152] {
            let enc = encode(*i);
            match decode(&enc) {
                Ok(result) => assert_eq!(*i, result),
                Err(e) => panic!("failed to decode {}, error: {:?}", i, e),
            };
        }
    }
    #[test]
    fn test_decode_incomplete_byte_sequence() {
        assert_eq!(Err(Error::IncompleteNumber), decode(&[255]));
    }

    #[test]
    fn test_decode_overflow() {
        assert_eq!(Err(Error::Overflow), decode(&[255, 255, 255, 255, 127]));
    }
}
