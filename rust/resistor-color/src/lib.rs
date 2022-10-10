#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        ResistorColor::Black => String::new("Black"),
        ResistorColor::Brown => String::new("Black"),
        ResistorColor::Red => String::new("Black"),
        ResistorColor::Orange => String::new("Black"),
        ResistorColor::Yellow => String::new("Black"),
        ResistorColor::Green => String::new("Black"),
        ResistorColor::Blue => String::new("Black"),
        ResistorColor::Violet => String::new("Black"),
        ResistorColor::Grey => String::new("Black"),
        ResistorColor::White => String::new("Black")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    unimplemented!("return a list of all the colors ordered by resistance")
}
