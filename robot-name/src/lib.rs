use core::iter::Cycle;
use core::ops::{Range, RangeInclusive};
use itertools::structs::Permutations;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct Robot(String);

lazy_static! {
    // In order to have a static mutable variable, we need a Mutex.
    // https://stackoverflow.com/a/27826181/839733.
    // Note that we cycle the iterators forever.
    static ref LETTERS_PERM: Mutex<Cycle<Permutations<RangeInclusive<u8>>>> =
        Mutex::new((b'A'..=b'Z').permutations(2).cycle());
    static ref DIGITS_PERM: Mutex<Cycle<Permutations<Range<u8>>>> =
        Mutex::new((0..10).permutations(3).cycle());
}

impl Robot {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Robot::new_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = Robot::new_name();
    }

    fn new_name() -> String {
        let x = (*LETTERS_PERM.lock().unwrap()).next().unwrap();
        let y = (*DIGITS_PERM.lock().unwrap()).next().unwrap();

        x.into_iter()
            .chain(y.into_iter().map(|i| i + b'0'))
            .map(|c| c as char)
            .collect::<String>()
    }
}
