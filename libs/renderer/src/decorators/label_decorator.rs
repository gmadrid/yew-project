use crate::decorators::CssMunger;
use grids::GridTrait;

mod emptylabels;
mod flatlabels;
mod mergedflatlabels;
mod roundlabels;

pub use emptylabels::EmptyLabels;
pub use flatlabels::FlatLabels;
pub use mergedflatlabels::MergedFlatLabels;
pub use roundlabels::RoundLabels;

pub trait LabelDecorator {
    fn register(&self, _munger: &CssMunger) {}

    fn left(&self, _grid: &dyn GridTrait, _row: usize) -> Option<(String, Vec<&'static str>)> {
        None
    }

    fn right(&self, _grid: &dyn GridTrait, _row: usize) -> Option<(String, Vec<&'static str>)> {
        None
    }

    fn has_bot(&self) -> bool {
        false
    }

    fn bot(
        &self,
        _grid: &dyn GridTrait,
        _col: usize,
    ) -> Option<(String, usize, Vec<&'static str>)> {
        None
    }
}
