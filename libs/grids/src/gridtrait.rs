use super::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum GridId {
    Main,
    LayerOne,
    LayerTwo,
    SmallOne,
    SmallTwo,
    Combined,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct CellId {
    pub grid_id: GridId,
    pub row: usize,
    pub col: usize,
}

impl<'a> CellId {
    pub fn new(grid_id: GridId, row: usize, col: usize) -> CellId {
        CellId { grid_id, row, col }
    }
}

// A Grid is essentially a matrix of Color values.
// It represents a knitting pattern.
// It also include a facility for naming grids.
pub trait GridTrait {
    fn grid_id(&self) -> GridId;
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
