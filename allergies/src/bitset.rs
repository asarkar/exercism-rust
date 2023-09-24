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
    pub fn to_int(&self) -> u8 {
        self.bits
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.bits)
    }
}
