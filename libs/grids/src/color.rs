use serde::{Deserialize, Serialize};

/// Color represents different color values in CSS.
///
/// Right now, there are a handful of colors which are explicitly listed - they are all
/// named CSS colors.
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
    /// Returns `true` if the color is [White].
    /// In many cases, we treat White as 'off' and any other color as 'on'.
    /// This allows us to toggle colors. See [std::ops::Not for Color].
    pub fn is_white(self) -> bool {
        self == Color::White
    }
}

impl Default for Color {
    /// Returns [White].
    fn default() -> Self {
        Color::White
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    /// In many cases, it's useful to treat a Color as 'on' or 'off.'
    /// We assume that [White] is 'off' and any other color is 'on.'
    /// The default 'on' value is [Gray].
    ///
    /// ```
    /// assert_eq!(!Color::White, Color::Gray);
    /// assert_eq!(!Color::Gray, Color::White);
    /// assert_eq!(!Color::Blue, Color::White);
    /// assert_eq!(!Color::Red, Color::White);
    /// ```
    fn not(self) -> Self::Output {
        if self.is_white() {
            Color::Gray
        } else {
            Color::White
        }
    }
}

impl ToString for Color {
    /// Returns the CSS color for the [Color].
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
