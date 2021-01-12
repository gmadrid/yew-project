use crate::decorators::{ClassDecorator, CssMunger};
use grids::{compute_base_indices, Color, GridTrait};

static ONCE: std::sync::Once = std::sync::Once::new();

#[derive(Default)]
pub struct MetagridDecorator {
    row_base_indices: Vec<u8>,
    col_base_indices: Vec<u8>,
}

impl MetagridDecorator {
    pub fn new(row_grid_cols: &[u8], col_grid_cols: &[u8]) -> Self {
        let row_base_indices = compute_base_indices(row_grid_cols);
        let col_base_indices = compute_base_indices(col_grid_cols);
        MetagridDecorator {
            row_base_indices,
            col_base_indices,
        }
    }
}

impl ClassDecorator for MetagridDecorator {
    fn register(&self, munger: &CssMunger) {
        ONCE.call_once(|| {
            munger.insert_rule("td.metathickleft{border-left:4px solid black} ");
            munger.insert_rule("td.metathicktop{border-top:4px solid black} ");
        });
    }

    fn cell_class(
        &self,
        _grid: &dyn GridTrait,
        row: usize,
        col: usize,
        _contents: Color,
    ) -> Vec<&'static str> {
        let mut vec = Vec::default();
        if col > 0 && self.col_base_indices[col] != self.col_base_indices[col - 1] {
            vec.push("metathickleft");
        }
        if row > 0 && self.row_base_indices[row] != self.row_base_indices[row - 1] {
            vec.push("metathicktop");
        }
        vec
    }
}
