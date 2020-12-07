use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Color {
    White,
    Gray,
    Blue,
    Orange,
    Yellow,
    Red,
    Green,
    Brown,

    // TODO: These currently don't do anything.
    Custom1,
    Custom2,
    Custom3,
    Custom4,
}

impl Color {
    pub fn is_white(self) -> bool {
        self == Color::White
    }

    pub fn style_str(self) -> String {
        // By default, printers don't bring the background-color.
        // color-adjust (and -webkit-print-color-adjust) override this setting in modern browsers.
        format!(
            "background: {}; color-adjust:exact; -webkit-print-color-adjust: exact;",
            self.to_string()
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is_white() {
            Color::Gray
        } else {
            Color::White
        }
    }
}

impl From<u8> for Color {
    fn from(f: u8) -> Color {
        match f {
            1 => Color::Gray,
            2 => Color::Blue,
            3 => Color::Orange,
            4 => Color::Yellow,
            5 => Color::Red,
            6 => Color::Green,
            7 => Color::Brown,
            _ => Color::White,
        }
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> u8 {
        match color {
            Color::Gray => 1,
            Color::Blue => 2,
            Color::Orange => 3,
            Color::Yellow => 4,
            Color::Red => 5,
            Color::Green => 6,
            Color::Brown => 7,
            Color::White => 8,
            _ => 8,
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::White => "white".to_owned(),
            Color::Gray => "darkgray".to_owned(),
            Color::Blue => "blue".to_owned(),
            Color::Orange => "orange".to_owned(),
            Color::Yellow => "yellow".to_owned(),
            Color::Red => "red".to_owned(),
            Color::Green => "green".to_owned(),
            Color::Brown => "brown".to_owned(),
            _ => Color::White.to_string(),
        }
    }
}
