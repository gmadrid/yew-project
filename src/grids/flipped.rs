use super::GridTrait;

pub struct FlippedGrid<'a, G>
where
    G: GridTrait<bool>,
{
    grid: &'a G,
}

impl<'a, G> FlippedGrid<'a, G>
where
    G: GridTrait<bool>,
{
    pub fn new(grid: &'a G) -> FlippedGrid<'a, G> {
        FlippedGrid { grid }
    }
}

impl<'a, G> GridTrait<bool> for FlippedGrid<'a, G>
where
    G: GridTrait<bool>,
{
    fn num_rows(&self) -> usize {
        self.grid.num_rows()
    }

    fn num_cols(&self) -> usize {
        self.grid.num_cols()
    }

    fn cell(&self, row: usize, col: usize) -> bool {
        self.grid.cell(row, self.num_cols() - col - 1)
    }

    fn set_cell(&mut self, _: usize, _: usize, _: bool) {
        unimplemented!("FlippedGrid is not mutable")
    }

    fn clear(&mut self) {
        unimplemented!("FlippedGrid is not mutable")
    }
}
