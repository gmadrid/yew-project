use crate::{GridId, GridTrait, TransposedGrid};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShiftDirection {
    Left,
    Right,
}

fn inclusive_iter(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    if start <= end {
        Box::new(start..end + 1)
    } else {
        Box::new((end..start + 1).rev())
    }
}

pub fn shift_rows(grid: &mut impl GridTrait, direction: ShiftDirection) {
    // offset: amount to add to 'from' index to get 'to' index
    // tmp_from: we have to copy one value into a temp variable to avoid overwriting it.
    //           this is the index from which to get that temp value.
    // tmp_to: the index to copy the temp value to
    // start: start indexing at this column, inclusive.
    // end: end indexing at this column, inclusive.
    let (offset, tmp_from, tmp_to, start, end) = match direction {
        ShiftDirection::Left => {
            // Since grid coords are usize, we cannot have a negative offset.
            let offset = grid.num_cols() - 1;
            (offset, 0, grid.num_cols() - 1, 1, grid.num_cols() - 1)
        }
        ShiftDirection::Right => (1, grid.num_cols() - 1, 0, grid.num_cols() - 2, 0),
    };

    for row in 0..grid.num_rows() {
        let temp = grid.cell(row, tmp_from);
        for from_col in inclusive_iter(start, end) {
            let to_col = (from_col + offset) % grid.num_cols();
            let from_value = grid.cell(row, from_col);

            grid.set_cell(row, to_col, from_value);
        }
        grid.set_cell(row, tmp_to, temp);
    }
}

pub fn shift_cols(grid: &mut impl GridTrait, direction: ShiftDirection) {
    let mut transpose = TransposedGrid::new(GridId::Temp, grid);
    shift_rows(&mut transpose, direction);
}
