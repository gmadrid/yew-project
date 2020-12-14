//mod biggrid;
mod color;
mod colorgrid;
mod flipped;
mod tiled;

pub use color::Color;

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

// A trait for grids that have a Grid id. Not essential for most grids.
pub trait HasGridId {
    fn grid_id(&self) -> &str;
}

pub use colorgrid::ColorGrid;
pub use flipped::FlippedGrid;
pub use tiled::TiledGrid;

const MAX_GRID_WIDTH: usize = 256;
const MAX_GRID_HEIGHT: usize = 256;
