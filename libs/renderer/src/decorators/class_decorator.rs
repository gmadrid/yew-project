mod borderedcell;
pub use borderedcell::{BorderedCellDecorator, ThickBorders};

mod cell_size;
pub use cell_size::{RegularSizedTableDecorator, SmallSizedTableDecorator};

mod merged_borders;
pub use merged_borders::MergedBorderDecorator;

mod metagrid;
pub use metagrid::MetagridDecorator;

mod printable;
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
