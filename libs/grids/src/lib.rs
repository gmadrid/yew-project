//! At it's core, a knitting pattern is a grid of stitches and stitch colors.
//! We represent all of these in a Grid - by which we just mean something that
//! implements [GridTrait].
//!
//! The most common grid is [SimpleGrid] which is just a grid of [Color].
//! ```
//! // Create a simple grid with 15 rows of 10 columns.
//! let grid = SimpleGrid::new(GridId::Main, 15, 10);
//! ```
//!
//! To some extent, grids can be composed, so that we can perform operations on a grid.
//! For example, this grid flips the underlying grid from left to right.
//! ```
//! let flipped = FlippedGrid(GridId::Flipped, &grid);
//! assert_eq!(grid.cell(0, 0), flipped.cell(0, 14));
//! ```
//!
//! Every grid gets a [GridId] to identify it. These have no meaning, and nothing enforces
//! uniqueness. It is up to the application to assign meaning to them.
//!
//! Finally, every cell in a grid has a [CellId]. This allows a cell to be uniquely identified
//! in things like messages. If a grid transforms or operates on another grid, the ```CellId```
//! associated with the cell will map down to the ```CellId``` in the base grid.
//! ```
//! assert_eq!(grid.cell_id(0, 0), flipped.cell_id(0, 14));
//! assert_eq!(flipped.cell_id(0, 14), CellId::new(GridId::Main, 0, 0));
//! ```

mod color;
pub use color::Color;

mod gridtrait;
pub use gridtrait::{CellId, GridId, GridTrait};

mod impls;
pub use impls::{
    BigGrid, FlippedGrid, InvertedGrid, MergedGrid, MetaGrid, SimpleGrid, TransposedGrid,
};

mod tools;
pub use tools::{shift_cols, shift_rows, ShiftDirection};
