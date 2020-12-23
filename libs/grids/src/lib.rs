mod color;
mod gridtrait;
mod hasgridid;
mod impls;

pub use color::Color;
pub use gridtrait::{CellId, GridId, GridTrait};
pub use hasgridid::HasGridId;

pub use impls::{InvertedGrid, MergedGrid, MetaGrid, SimpleGrid};
