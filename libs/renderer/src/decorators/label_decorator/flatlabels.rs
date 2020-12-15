use super::LabelDecorator;
use grids::GridTrait;

pub struct FlatLabels;

impl Default for FlatLabels {
    fn default() -> Self {
        FlatLabels
    }
}

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

    fn bot(&self, _grid: &dyn GridTrait, _col: usize) -> Option<(String, Vec<&'static str>)> {
        None
    }
}
