pub trait GridTrait<T> {
    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;

    fn cell(&self, row: usize, col: usize) -> T;
    fn set_cell(&mut self, row: usize, col: usize, value: T);

    fn clear(&mut self);
}
