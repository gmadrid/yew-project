use crate::{Color, GridId, GridTrait};
use serde::{Deserialize, Serialize};

const MAX_GRID_HEIGHT: usize = 100;
const MAX_GRID_WIDTH: usize = 100;

#[derive(Serialize, Deserialize)]
pub struct BigGrid {
    id: GridId,
    cells: Vec<Color>,
    height: usize,
    width: usize,
}

impl BigGrid {
    pub fn new(id: GridId, height: usize, width: usize) -> BigGrid {
        BigGrid {
            id,
            cells: vec![Default::default(); MAX_GRID_WIDTH * MAX_GRID_HEIGHT],
            height,
            width,
        }
    }

    fn coords_to_index(&self, row: usize, col: usize) -> usize {
        row * MAX_GRID_WIDTH + col
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        if rows > MAX_GRID_HEIGHT {
            panic!("rows ({}) must be <= {}", rows, MAX_GRID_HEIGHT);
        }
        if cols > MAX_GRID_WIDTH {
            panic!("cols ({}) must be <= {}", cols, MAX_GRID_WIDTH);
        }
        self.height = rows;
        self.width = cols;
    }
}

impl GridTrait for BigGrid {
    fn grid_id(&self) -> GridId {
        self.id
    }

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
