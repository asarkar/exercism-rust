mod bitset;

use bitset::BitSet;
use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::collections::HashMap;

pub struct Allergies {
    allergies: BitSet,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, IntEnum, Sequence, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let nums: Vec<u8> = all::<Allergen>().map(|a| a.int_value()).collect();
        let bits = subset_sum(
            &nums[..],
            0,
            score & 255,
            &mut BitSet::new(0),
            &mut HashMap::new(),
        );
        if let Some(b) = bits {
            Allergies { allergies: b }
        } else {
            Allergies {
                allergies: BitSet::new(0),
            }
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        all::<Allergen>()
            .enumerate()
            .filter(|&(i, _)| self.allergies.is_set(i))
            .map(|(_, a)| a)
            .collect()
    }
}

// Try including and excluding every number. Use a bit set
// to keep track of which numbers are selected.
fn subset_sum(
    nums: &[u8],
    i: usize,
    remaining: u32,
    bits: &mut BitSet,
    memo: &mut HashMap<(usize, u32), u8>,
) -> Option<BitSet> {
    if remaining == 0 {
        return Some(bits.clone());
    }
    if i >= nums.len() || bits.is_set(i) {
        return None;
    }
    if let Some(&b) = memo.get(&(i, remaining)) {
        return Some(BitSet::new(b));
    }

    let mut found = None;
    if (nums[i] as u32) <= remaining {
        bits.set(i);
        found = subset_sum(nums, i + 1, remaining - (nums[i] as u32), bits, memo);
    }
    bits.clear(i);
    if found.is_none() {
        found = subset_sum(nums, i + 1, remaining, bits, memo);
    }
    memo.insert(
        (i, remaining),
        // Without as_ref(), found is moved here, and can't be used as a return value
        found.as_ref().map(|b| b.to_int()).unwrap_or_default(),
    );

    found
}
