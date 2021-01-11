use crate::{CellId, GridId, GridTrait};

pub struct TransposedGrid<'a, GRID: GridTrait> {
    id: GridId,
    grid: &'a mut GRID,
}

impl<'a, GRID: GridTrait> TransposedGrid<'a, GRID> {
    pub fn new(id: GridId, grid: &'a mut GRID) -> Self {
        TransposedGrid { id, grid }
    }
}

impl<'a, GRID: GridTrait> GridTrait for TransposedGrid<'a, GRID> {
    fn grid_id(&self) -> GridId {
        self.id
    }

    fn cell_id(&self, row: usize, col: usize) -> CellId {
        self.grid.cell_id(col, row)
    }

    fn num_rows(&self) -> usize {
        self.grid.num_cols()
    }

    fn num_cols(&self) -> usize {
        self.grid.num_rows()
    }

    fn cell(&self, row: usize, col: usize) -> crate::Color {
        self.grid.cell(col, row)
    }

    fn set_cell(&mut self, row: usize, col: usize, value: crate::Color) {
        self.grid.set_cell(col, row, value)
    }
}
