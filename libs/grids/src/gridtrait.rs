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
    Flipped,
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
/// `GridTrait` is the base abstraction in the 'grids' crate. It represents a grid of cells.
///
/// A struct that implements `GridTrait` has the following:
/// * an id, represented by [GridId]
/// * a rectangular matrix, each cell of which has:
///   * a [Color]
///   * an id, represented by a [CellId]. If an implementation of GridTrait transforms an
///     underlying grid, the cell ids are those of the base grid.
pub trait GridTrait {
    /// Returns the [GridId] associated with this grid.
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
