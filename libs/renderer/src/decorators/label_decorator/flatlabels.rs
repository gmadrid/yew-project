use super::LabelDecorator;
use grids::GridTrait;

#[derive(Default)]
pub struct FlatLabels;

impl LabelDecorator for FlatLabels {
    fn left(&self, grid: &dyn GridTrait, row: usize) -> Option<(String, Vec<&'static str>)> {
        let label = grid.num_rows() - row;
        if label % 2 == 0 {
            Some((label.to_string(), vec![]))
        } else {
            Some(("".to_string(), vec![]))
        }
    }

    fn right(&self, grid: &dyn GridTrait, row: usize) -> Option<(String, Vec<&'static str>)> {
        let label = grid.num_rows() - row;
        if label % 2 == 1 {
            Some((label.to_string(), vec![]))
        } else {
            Some(("".to_string(), vec![]))
        }
    }

    fn has_bot(&self) -> bool {
        true
    }

    fn bot(&self, grid: &dyn GridTrait, col: usize) -> Option<(String, Vec<&'static str>)> {
        let label = grid.num_cols() - col;
        Some((label.to_string(), vec![]))
    }
}
