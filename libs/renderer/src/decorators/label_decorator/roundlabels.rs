use super::LabelDecorator;
use grids::GridTrait;

#[derive(Default)]
pub struct RoundLabels;

impl LabelDecorator for RoundLabels {
    fn right(&self, grid: &dyn GridTrait, row: usize) -> Option<(String, Vec<&'static str>)> {
        let label = grid.num_rows() - row;
        Some((label.to_string(), vec![]))
    }

    fn has_bot(&self) -> bool {
        true
    }

    fn bot(&self, grid: &dyn GridTrait, col: usize) -> Option<(String, Vec<&'static str>)> {
        let label = grid.num_cols() - col;
        Some((label.to_string(), vec![]))
    }
}
