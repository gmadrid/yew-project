use super::grids::Color;
use super::grids::GridTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SimpleGrid {
    cells: Vec<Color>,
    height: usize,
    width: usize,
}

impl SimpleGrid {
    pub fn new(height: usize, width: usize) -> SimpleGrid {
        SimpleGrid {
            cells: vec![Default::default(); height * width],
            height,
            width,
        }
    }

    fn coords_to_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl GridTrait for SimpleGrid {
    fn num_rows(&self) -> usize {
        self.height
    }

    fn num_cols(&self) -> usize {
        self.width
    }

    fn cell(&self, row: usize, col: usize) -> Color {
        let index = self.coords_to_index(row, col);
        self.cells[index]
    }

    fn set_cell(&mut self, row: usize, col: usize, value: Color) {
        let index = self.coords_to_index(row, col);
        self.cells[index] = value
    }

    fn clear(&mut self) {
        for index in 0..self.cells.len() {
            self.cells[index] = Default::default();
        }
    }
}
