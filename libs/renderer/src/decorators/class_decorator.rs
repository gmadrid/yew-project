mod borderedcell;
mod cell_size;
mod merged_borders;
mod printable;

pub use borderedcell::{BorderedCellDecorator, ThickBorders};
pub use cell_size::{RegularSizedTableDecorator, SmallSizedTableDecorator};
pub use merged_borders::MergedBorderDecorator;
pub use printable::PrintableColorDecorator;

use crate::decorators::CssMunger;
use grids::{Color, GridTrait};

pub trait ClassDecorator {
    fn register(&self, _munger: &CssMunger) {}

    fn cell_class(
        &self,
        _grid: &dyn GridTrait,
        _row: usize,
        _col: usize,
        _contents: Color,
    ) -> Vec<&'static str> {
        vec![]
    }

    fn label_class(&self, _grid: &dyn GridTrait, _row: usize) -> Vec<&'static str> {
        vec![]
    }
}
