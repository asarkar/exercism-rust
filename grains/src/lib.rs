pub fn square(s: u32) -> u64 {
    assert!((1..=64).contains(&s), "Square must be between 1 and 64");
    u64::pow(2, s - 1)
}

pub fn total() -> u64 {
    18_446_744_073_709_551_615
}
