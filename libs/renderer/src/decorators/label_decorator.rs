use grids::GridTrait;

mod emptylabels;
mod flatlabels;
mod roundlabels;

pub use emptylabels::EmptyLabels;
pub use flatlabels::FlatLabels;
pub use roundlabels::RoundLabels;

pub trait LabelDecorator {
    fn left(&self, _grid: &dyn GridTrait, _row: usize) -> Option<(String, Vec<&'static str>)> {
        None
    }

    fn right(&self, _grid: &dyn GridTrait, _row: usize) -> Option<(String, Vec<&'static str>)> {
        None
    }

    fn has_bot(&self) -> bool {
        false
    }

    fn bot(&self, _grid: &dyn GridTrait, _col: usize) -> Option<(String, Vec<&'static str>)> {
        None
    }
}
