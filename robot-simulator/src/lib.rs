use int_enum::IntEnum;
// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
// Having the directions represented as integers makes turning the
// robot much easier.
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub struct Robot {
    // Unfortunately, the direction() method returns a &Direction,
    // so, we are forced to store the direction enum and convert
    // back and forth from integer.
    dir: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { dir: d, x, y }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let orientation = (self.dir.int_value() + 1) % 4;
        let dir = Direction::from_int(orientation).unwrap();
        Robot { dir, ..self }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let orientation = if self.dir == Direction::North {
            Direction::West.int_value()
        } else {
            self.dir.int_value() - 1
        };
        let dir = Direction::from_int(orientation).unwrap();
        Robot { dir, ..self }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = match self.dir {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            _ => (self.x - 1, self.y),
        };
        Robot { x, y, ..self }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.as_bytes().iter().fold(self, |r, b| match b {
            b'R' => r.turn_right(),
            b'L' => r.turn_left(),
            _ => r.advance(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
