use crate::grids::{Color, GridTrait};

pub trait IndirectGrid {
    fn to_base(&self, row: usize, col: usize) -> (usize, usize);
}

pub struct MetaGrid<'a, GRID>
where
    GRID: GridTrait<Color>,
{
    grid: &'a GRID,
    row_indices: &'a [u8],
    col_indices: &'a [u8],
}

impl<'a, GRID> MetaGrid<'a, GRID>
where
    GRID: GridTrait<Color>,
{
    pub fn new(grid: &'a GRID, row_indices: &'a [u8], col_indices: &'a [u8]) -> Self {
        MetaGrid {
            grid,
            row_indices,
            col_indices,
        }
    }
}

impl<'a, GRID> IndirectGrid for MetaGrid<'a, GRID>
where
    GRID: GridTrait<Color>,
{
    fn to_base(&self, row: usize, col: usize) -> (usize, usize) {
        (
            base_index(row, self.row_indices),
            base_index(col, self.col_indices),
        )
    }
}

fn base_index(index: usize, vec: &[u8]) -> usize {
    let mut high = 0;
    for (base_idx, step) in vec.into_iter().enumerate() {
        high += step;
        if index < high as usize {
            return base_idx;
        }
    }
    panic!("too big")
}

impl<'a, GRID> GridTrait<Color> for MetaGrid<'a, GRID>
where
    GRID: GridTrait<Color>,
{
    fn num_rows(&self) -> usize {
        self.row_indices.iter().sum::<u8>() as usize
    }

    fn num_cols(&self) -> usize {
        self.col_indices.iter().sum::<u8>() as usize
    }

    fn cell(&self, row: usize, col: usize) -> Color {
        // TODO: You should speed this up by pre-computing these offsets.
        let base_x = base_index(row, &self.row_indices);
        let base_y = base_index(col, &self.col_indices);
        self.grid.cell(base_x, base_y)
    }

    fn set_cell(&mut self, _: usize, _: usize, _: Color) {
        // no-op
    }

    fn clear(&mut self) {
        // no-op
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indirection() {
        let indirect: Vec<u8> = vec![1, 1, 2, 2, 3, 2, 2, 1, 1];

        let tot = indirect.iter().sum();
        let base_indices = (0..tot)
            .map(|idx| base_index(idx as usize, &indirect))
            .collect::<Vec<_>>();

        assert_eq!(
            base_indices,
            vec![0, 1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 8]
        )
    }
}
