use crate::{CellId, Color, GridId, GridTrait};

pub struct MergedGrid<'a, GRIDONE, GRIDTWO>
where
    GRIDONE: GridTrait,
    GRIDTWO: GridTrait,
{
    id: GridId,
    grid_one: &'a GRIDONE,
    grid_two: &'a GRIDTWO,
}

impl<'a, GRIDONE, GRIDTWO> MergedGrid<'a, GRIDONE, GRIDTWO>
where
    GRIDONE: GridTrait,
    GRIDTWO: GridTrait,
{
    pub fn new(id: GridId, grid_one: &'a GRIDONE, grid_two: &'a GRIDTWO) -> Self {
        if grid_one.num_cols() != grid_two.num_cols() || grid_one.num_rows() != grid_two.num_rows()
        {
            panic!(
                "Grids must be same size. grid one: ({}, {}), grid_two: ({}, {}).",
                grid_one.num_rows(),
                grid_one.num_cols(),
                grid_two.num_rows(),
                grid_two.num_cols()
            );
        }
        MergedGrid {
            id,
            grid_one,
            grid_two,
        }
    }

    pub fn base_coords(&self, row: usize, col: usize) -> (usize, usize) {
        (row, col / 2)
    }

    pub fn grid_for_location(&self, _row: usize, col: usize) -> &dyn GridTrait {
        // grid_one is shown in the knit squares.
        // grid_two is shown in the purl squares.
        // This means that grid_one squares will be displayed to the right of grid_two.
        // (This may be slightly counter-intuitive, but that's the way charts work.)
        if col % 2 == 0 {
            self.grid_two
        } else {
            self.grid_one
        }
    }
}

impl<'a, GRIDONE, GRIDTWO> GridTrait for MergedGrid<'a, GRIDONE, GRIDTWO>
where
    GRIDONE: GridTrait,
    GRIDTWO: GridTrait,
{
    fn grid_id(&self) -> GridId {
        self.id
    }

    fn cell_id(&self, row: usize, col: usize) -> CellId {
        let (base_row, base_col) = self.base_coords(row, col);
        self.grid_for_location(row, col).cell_id(base_row, base_col)
    }

    fn num_rows(&self) -> usize {
        self.grid_one.num_rows()
    }

    fn num_cols(&self) -> usize {
        self.grid_one.num_cols() + self.grid_two.num_cols()
    }

    fn cell(&self, row: usize, col: usize) -> Color {
        let grid = self.grid_for_location(row, col);
        let (base_row, base_col) = self.base_coords(row, col);
        grid.cell(base_row, base_col)
    }

    fn set_cell(&mut self, _row: usize, _col: usize, _value: Color) {
        // no-op. MergedGrid is read-only.
    }

    fn clear(&mut self) {
        // no-op. MergedGrid is read-only.
    }
}
