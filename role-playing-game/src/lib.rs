// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health >= 1 {
            return None;
        }
        let p = Player {
            health: 100,
            mana: Some(100).filter(|_| self.level >= 10),
            level: self.level,
        };
        Some(p)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = if mana_cost > self.health {
                    0
                } else {
                    self.health - mana_cost
                };
                0
            }
            Some(m) if m < mana_cost => 0,
            Some(m) => {
                self.mana = Some(m - mana_cost);
                2 * mana_cost
            }
        }
    }
}
