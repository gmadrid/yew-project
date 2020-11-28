use super::gridtrait::GridTrait;
use super::{MAX_GRID_HEIGHT, MAX_GRID_WIDTH};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
    pub fn style_str(&self) -> String {
        format!("background: {}", self.to_string())
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
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::White => "white".to_owned(),
            Color::Gray => "gray".to_owned(),
            Color::Blue => "blue".to_owned(),
            Color::Orange => "orange".to_owned(),
            Color::Yellow => "yellow".to_owned(),
            Color::Red => "red".to_owned(),
            Color::Green => "green".to_owned(),
            Color::Brown => "brown".to_owned(),
        }
    }
}

pub struct ColorGrid {
    cells: Vec<Color>,
    num_rows: usize,
    num_cols: usize,
}

impl ColorGrid {
    pub fn new() -> ColorGrid {
        ColorGrid {
            num_rows: Default::default(),
            num_cols: Default::default(),
            cells: vec![Color::White; MAX_GRID_HEIGHT * MAX_GRID_WIDTH],
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.num_rows = rows;
        self.num_cols = cols;
    }
}

impl GridTrait<Color> for ColorGrid {
    fn num_rows(&self) -> usize {
        self.num_rows
    }

    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn cell(&self, row: usize, col: usize) -> Color {
        let index = row * MAX_GRID_WIDTH + col;
        self.cells[index]
    }

    fn set_cell(&mut self, row: usize, col: usize, value: Color) {
        let index = row * MAX_GRID_WIDTH + col;
        self.cells[index] = value;
    }

    fn clear(&mut self) {
        for index in 0..self.cells.len() {
            self.cells[index] = Color::Red;
        }
    }
}
