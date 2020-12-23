use crate::{CellId, Color, GridId, GridTrait};

pub struct InvertedGrid<'a, GRID>
where
    GRID: GridTrait,
{
    id: GridId,
    grid: &'a GRID,
}

impl<'a, GRID> InvertedGrid<'a, GRID>
where
    GRID: GridTrait,
{
    pub fn new(id: GridId, grid: &'a GRID) -> Self {
        InvertedGrid { id, grid }
    }
}

impl<'a, GRID> GridTrait for InvertedGrid<'a, GRID>
where
    GRID: GridTrait,
{
    fn grid_id(&self) -> GridId {
        self.id
    }

    fn cell_id(&self, row: usize, col: usize) -> CellId {
        self.grid.cell_id(row, col)
    }

    fn num_rows(&self) -> usize {
        self.grid.num_rows()
    }

    fn num_cols(&self) -> usize {
        self.grid.num_cols()
    }

    fn cell(&self, row: usize, col: usize) -> Color {
        !self.grid.cell(row, col)
    }

    fn set_cell(&mut self, _row: usize, _col: usize, _value: Color) {
        // no-op. Read-only.
    }

    fn clear(&mut self) {
        // no-op. Read-only.
    }
}
