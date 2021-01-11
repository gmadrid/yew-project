use super::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum GridId {
    Main,
    LayerOne,
    LayerTwo,
    SmallOne,
    SmallTwo,
    Combined,
    Flipped,
    Temp,
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

    /// Returns the [CellId] for the cell in the _base_ grid.
    fn cell_id(&self, row: usize, col: usize) -> CellId {
        CellId::new(self.grid_id(), row, col)
    }

    /// Returns the number of rows in the grid.
    fn num_rows(&self) -> usize;
    /// Returns the number of cols in the grid.
    fn num_cols(&self) -> usize;

    /// Returns the [Color] at the (`row`, `col`) location in the grid.
    fn cell(&self, row: usize, col: usize) -> Color;
    /// Sets the [Color] at the (`row`, `col`) location in the grid.
    fn set_cell(&mut self, row: usize, col: usize, value: Color);

    /// 'Clears' the grid.
    ///
    /// The default implementation sets all of the cells to [Color::default].
    /// Implementations are free to interpret this however makes sense, or may write their own
    /// version for efficiency.
    fn clear(&mut self) {
        for row in 0..self.num_rows() {
            for col in 0..self.num_cols() {
                self.set_cell(row, col, Color::default());
            }
        }
    }
}

pub trait ToBaseTrait {
    /// For any grid that modifies or filters an underlying grid, returns the (row, col)
    /// in the _base_ grid that corresponds to the cell in the filtered grid.
    fn to_base(&self, row: usize, col: usize) -> (usize, usize) {
        (row, col)
    }
}
