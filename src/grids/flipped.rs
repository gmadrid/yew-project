use super::{Color, GridTrait};

pub struct FlippedGrid<'a, G>
where
    G: GridTrait,
{
    grid: &'a G,
}

impl<'a, G> FlippedGrid<'a, G>
where
    G: GridTrait,
{
    pub fn new(grid: &'a G) -> FlippedGrid<'a, G> {
        FlippedGrid { grid }
    }
}

impl<'a, G> GridTrait for FlippedGrid<'a, G>
where
    G: GridTrait,
{
    fn num_rows(&self) -> usize {
        self.grid.num_rows()
    }

    fn num_cols(&self) -> usize {
        self.grid.num_cols()
    }

    fn cell(&self, row: usize, col: usize) -> Color {
        self.grid.cell(row, self.num_cols() - col - 1)
    }

    fn set_cell(&mut self, _: usize, _: usize, _: Color) {
        unimplemented!("FlippedGrid is not mutable")
    }

    fn clear(&mut self) {
        unimplemented!("FlippedGrid is not mutable")
    }
}
