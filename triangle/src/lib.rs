use std::cmp::PartialOrd;
use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Copy + Add<Output = T> + Default,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|&x| x == T::default()) {
            return None;
        }
        if (sides[0] + sides[1] < sides[2])
            || (sides[0] + sides[2] < sides[1])
            || (sides[1] + sides[2] < sides[0])
        {
            return None;
        }
        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        (self.sides[0] == self.sides[1]) && (self.sides[1] == self.sides[2])
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        (self.sides[0] == self.sides[1])
            || (self.sides[0] == self.sides[2])
            || (self.sides[1] == self.sides[2])
    }
}
