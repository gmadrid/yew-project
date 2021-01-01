mod color;
mod gridtrait;
mod impls;
mod traits;

pub use color::Color;
pub use gridtrait::{CellId, GridId, GridTrait};
pub use traits::{HasGridId, HasGrids};

pub use impls::{InvertedGrid, MergedGrid, MetaGrid, SimpleGrid};
