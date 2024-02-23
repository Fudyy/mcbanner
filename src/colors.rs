#[derive(Debug)]
pub enum MCColor {
    White,
    LightGray,
    Gray,
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Cyan,
    LightBlue,
    Blue,
    Purple,
    Magenta,
    Pink,
}

impl MCColor {
    pub fn rgb(&self) -> [u8; 3] {
        match self {
            MCColor::White => [249, 255, 254],
            MCColor::LightGray => [157, 157, 151],
            MCColor::Gray => [71, 79, 82],
            MCColor::Black => [29, 29, 33],
            MCColor::Brown => [131, 84, 50],
            MCColor::Red => [176, 46, 38],
            MCColor::Orange => [249, 128, 29],
            MCColor::Yellow => [254, 216, 61],
            MCColor::Lime => [128, 199, 31],
            MCColor::Green => [94, 124, 22],
            MCColor::Cyan => [22, 156, 156],
            MCColor::LightBlue => [58, 179, 218],
            MCColor::Blue => [60, 68, 170],
            MCColor::Purple => [137, 50, 184],
            MCColor::Magenta => [199, 78, 189],
            MCColor::Pink => [243, 139, 170],
        }
    }
}