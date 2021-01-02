use super::LabelDecorator;
use grids::GridTrait;

#[derive(Default)]
pub struct FlatLabels {
    row_offset: usize,
    col_offset: usize,
}

impl FlatLabels {
    pub fn starting_at(row_start: usize, col_start: usize) -> FlatLabels {
        FlatLabels {
            row_offset: row_start - 1,
            col_offset: col_start - 1,
        }
    }
}

impl LabelDecorator for FlatLabels {
    fn left(&self, grid: &dyn GridTrait, row: usize) -> Option<(String, Vec<&'static str>)> {
        let label = grid.num_rows() - row + self.row_offset;
        if label % 2 == 0 {
            Some((label.to_string(), vec![]))
        } else {
            Some(("".to_string(), vec![]))
        }
    }

    fn right(&self, grid: &dyn GridTrait, row: usize) -> Option<(String, Vec<&'static str>)> {
        let label = grid.num_rows() - row + self.row_offset;
        if label % 2 == 1 {
            Some((label.to_string(), vec![]))
        } else {
            Some(("".to_string(), vec![]))
        }
    }

    fn has_bot(&self) -> bool {
        true
    }

    fn bot(&self, grid: &dyn GridTrait, col: usize) -> Option<(String, usize, Vec<&'static str>)> {
        let label = grid.num_cols() - col + self.col_offset;
        Some((label.to_string(), 1, vec![]))
    }
}
