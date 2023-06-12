use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

impl Category {
    pub fn index(&self) -> u8 {
        *self as u8
    }
}

type Dice = [u8; 5];
pub fn score(dice: Dice, category: Category) -> u8 {
    let freq = dice.into_iter().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0u8) += 1;
        hm
    });
    let n = freq.len();
    match category {
        c if c.index() < 6 => {
            let k = c.index() + 1;
            *freq.get(&k).unwrap_or(&0) * k
        }
        Category::FullHouse if n == 2 => freq
            .iter()
            .filter(|(_, v)| **v == 2 || **v == 3)
            .fold(0, |acc, (k, v)| acc + k * v),
        Category::FourOfAKind => freq
            .iter()
            .filter(|(_, v)| **v >= 4)
            .fold(0, |_, (k, _)| k * 4),
        Category::LittleStraight if n == 5 && !freq.contains_key(&6) => 30,
        Category::BigStraight if n == 5 && !freq.contains_key(&1) => 30,
        Category::Choice => dice.iter().sum(),
        Category::Yacht if n == 1 => 50,
        _ => 0,
    }
}
