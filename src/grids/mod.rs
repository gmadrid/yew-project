//mod biggrid;
mod color;
mod colorgrid;
mod flipped;
mod gridtrait;
mod tiled;

pub use color::Color;
pub use gridtrait::GridTrait;

pub use colorgrid::ColorGrid;
pub use flipped::FlippedGrid;
pub use tiled::TiledGrid;

const MAX_GRID_WIDTH: usize = 256;
const MAX_GRID_HEIGHT: usize = 256;
