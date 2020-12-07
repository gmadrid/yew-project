use super::gridtrait::GridTrait;
use super::{Color, MAX_GRID_HEIGHT, MAX_GRID_WIDTH};

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

impl GridTrait for ColorGrid {
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
