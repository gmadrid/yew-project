use super::LabelDecorator;
use crate::decorators::CssMunger;
use grids::GridTrait;
use tools::logic::if_option;

static ONCE: std::sync::Once = std::sync::Once::new();

#[derive(Default)]
pub struct MergedFlatLabels {
    row_offset: usize,
    col_offset: usize,
}

impl MergedFlatLabels {
    pub fn starting_at(row_start: usize, col_start: usize) -> Self {
        MergedFlatLabels {
            row_offset: row_start - 1,
            col_offset: col_start - 1,
        }
    }
}

impl LabelDecorator for MergedFlatLabels {
    fn register(&self, munger: &CssMunger) {
        ONCE.call_once(|| munger.insert_rule("th.mleft{border-left:1px solid black} "));
    }

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
        if_option(col % 2 == 0, || {
            let label = (grid.num_cols() - col) / 2 + self.col_offset;
            let mut classes = vec![];
            if col != 0 {
                classes.push("mleft");
            }
            (label.to_string(), 2, classes)
        })
    }
}
