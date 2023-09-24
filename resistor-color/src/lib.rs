use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::fmt::{self, Debug};

#[repr(u8)]
#[derive(Clone, Copy, Debug, IntEnum, Sequence, PartialEq)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    all::<ResistorColor>()
        .find(|c| (c.int_value() as usize) == value)
        .map(|c| c.to_string())
        .unwrap_or_else(|| "value out of range".to_string())
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors = all::<ResistorColor>().collect::<Vec<_>>();
    colors.sort_by_key(|x| x.int_value());
    colors
}
