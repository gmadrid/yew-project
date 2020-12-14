use super::Color;

// A Grid is essentially a matrix of Color values.
// It represents a knitting pattern.
// It also include a facility for naming grids.
pub trait GridTrait {
    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;

    fn cell(&self, row: usize, col: usize) -> Color;
    fn set_cell(&mut self, row: usize, col: usize, value: Color);

    fn clear(&mut self);
}
