mod biggrid;
mod colorgrid;
mod flipped;
mod gridtrait;

pub use colorgrid::{Color, ColorGrid};
pub use flipped::FlippedGrid;
pub use gridtrait::GridTrait;

const MAX_GRID_WIDTH: usize = 256;
const MAX_GRID_HEIGHT: usize = 256;