use super::Color;

pub struct CellId<'a> {
    pub grid_id: &'a str,
    pub row: usize,
    pub col: usize,
}

impl<'a> CellId<'a> {
    pub fn new(grid_id: &'a str, row: usize, col: usize) -> CellId {
        CellId { grid_id, row, col }
    }
}

// A Grid is essentially a matrix of Color values.
// It represents a knitting pattern.
// It also include a facility for naming grids.
pub trait GridTrait {
    fn grid_id(&self) -> &str;
    fn cell_id(&self, row: usize, col: usize) -> CellId {
        CellId::new(self.grid_id(), row, col)
    }

    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;

    fn cell(&self, row: usize, col: usize) -> Color;
    fn set_cell(&mut self, row: usize, col: usize, value: Color);

    fn clear(&mut self);
}

pub trait ToBaseTrait {
    fn to_base(&self, row: usize, col: usize) -> (usize, usize) {
        (row, col)
    }
}
