mod color;
mod gridtrait;
mod impls;

pub use color::Color;
pub use gridtrait::{CellId, GridId, GridTrait};

pub use impls::{InvertedGrid, MergedGrid, MetaGrid, SimpleGrid};
