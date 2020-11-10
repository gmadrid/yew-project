use super::gridtrait::GridTrait;

pub struct SimpleGrid<T> {
    cells: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> SimpleGrid<T>
where
    T: Copy + Default,
{
    pub fn new(height: usize, width: usize) -> SimpleGrid<T> {
        SimpleGrid {
            cells: vec![T::default(); height * width],
            height,
            width,
        }
    }

    fn coords_to_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl<T> SimpleGrid<T>
where
    T: std::ops::Not<Output = T> + Copy + Default,
{
    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let index = self.coords_to_index(row, col);
        self.cells[index] = !self.cells[index];
    }
}

impl<T> GridTrait<T> for SimpleGrid<T>
where
    T: Copy + Default,
{
    fn num_rows(&self) -> usize {
        self.height
    }

    fn num_cols(&self) -> usize {
        self.width
    }

    fn cell(&self, row: usize, col: usize) -> T {
        let index = self.coords_to_index(row, col);
        self.cells[index]
    }

    fn set_cell(&mut self, row: usize, col: usize, value: T) {
        let index = self.coords_to_index(row, col);
        self.cells[index] = value
    }

    fn clear(&mut self) {
        for index in 0..self.cells.len() {
            self.cells[index] = T::default();
        }
    }
}
