use std::fmt;

#[derive(Clone)]
pub struct BitSet {
    bits: u8,
}

impl BitSet {
    pub fn new(bits: u8) -> Self {
        BitSet { bits }
    }
    pub fn set(&mut self, i: usize) {
        let mask = 1 << i;
        self.bits |= mask
    }
    pub fn clear(&mut self, i: usize) {
        let mask = 1 << i;
        self.bits &= !mask
    }
    #[allow(dead_code)]
    pub fn toggle(&mut self, i: usize) {
        let mask = 1 << i;
        self.bits ^= mask
    }
    pub fn is_set(&self, i: usize) -> bool {
        let mask = 1 << i;
        (self.bits & mask) > 0
    }
    pub fn all_set(&self) -> bool {
        self.to_int() == 255
    }
    pub fn to_int(&self) -> u8 {
        self.bits
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        let mut b = BitSet::new(0);
        assert_eq!(b.to_int(), 0);

        b.set(0);
        assert_eq!(b.to_int(), 1);
        assert!(b.is_set(0));

        b.set(1);
        assert_eq!(b.to_int(), 3);
        assert!(b.is_set(1));

        b.clear(1);
        assert_eq!(b.to_int(), 1);
        assert!(!b.is_set(1));

        b.toggle(1);
        assert_eq!(b.to_int(), 3);
        assert!(b.is_set(1));
    }

    #[test]
    fn test_clone() {
        let mut b = BitSet::new(3);
        let mut c = b.clone();

        b.clear(1);
        assert_eq!(c.to_int(), 3);

        c.clear(0);
        assert_eq!(b.to_int(), 1);
    }

    #[test]
    fn test_to_string() {
        let mut b = BitSet::new(0);
        assert_eq!(format!("{}", b), "0");

        b.set(0);
        assert_eq!(format!("{}", b), "1");

        b.set(1);
        assert_eq!(format!("{}", b), "11");
    }
}
