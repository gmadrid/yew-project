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
}

impl Color {
    pub fn is_white(self) -> bool {
        self == Color::White
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    /// !White = Gray.
    /// !AnyOtherColor = White
    fn not(self) -> Self::Output {
        if self.is_white() {
            Color::Gray
        } else {
            Color::White
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
        }
    }
}
