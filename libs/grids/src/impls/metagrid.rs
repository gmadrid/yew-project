use crate::gridtrait::ToBaseTrait;
use crate::{GridId, GridTrait};

pub struct MetaGrid<'a, GRID: GridTrait> {
    id: GridId,
    grid: &'a GRID,

    base_row_indices: Vec<u8>,
    base_col_indices: Vec<u8>,
}

fn compute_base_indices(indices: &[u8]) -> Vec<u8> {
    let mut vec = Vec::default();

    let mut base_index = 0;
    for index in indices {
        for _ in 0..*index {
            vec.push(base_index);
        }
        base_index += 1;
    }

    vec
}

impl<'a, GRID: GridTrait> MetaGrid<'a, GRID> {
    pub fn new(
        id: GridId,
        grid: &'a GRID,
        row_indices: &'a [u8],
        col_indices: &'a [u8],
    ) -> MetaGrid<'a, GRID> {
        let base_row_indices = compute_base_indices(row_indices);
        let base_col_indices = compute_base_indices(col_indices);

        MetaGrid {
            id,
            grid,

            base_row_indices,
            base_col_indices,
        }
    }
}

impl<'a, GRID: GridTrait> ToBaseTrait for MetaGrid<'a, GRID> {
    fn to_base(&self, row: usize, col: usize) -> (usize, usize) {
        (
            self.base_row_indices[row] as usize,
            self.base_col_indices[col] as usize,
        )
    }
}

impl<'a, GRID: GridTrait> GridTrait for MetaGrid<'a, GRID> {
    fn grid_id(&self) -> GridId {
        self.id
    }

    fn num_rows(&self) -> usize {
        self.base_row_indices.len()
    }

    fn num_cols(&self) -> usize {
        self.base_col_indices.len()
    }

    fn cell(&self, row: usize, col: usize) -> crate::Color {
        let (base_row, base_col) = self.to_base(row, col);
        self.grid.cell(base_row, base_col)
    }

    fn set_cell(&mut self, _row: usize, _col: usize, _value: crate::Color) {
        panic!("MetaGrid is read-only.")
    }

    fn clear(&mut self) {
        panic!("MetaGrid is read-only.")
    }
}
