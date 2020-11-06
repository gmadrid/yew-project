struct VecWrapper<'a, T> {
    vec: &'a Vec<T>,
    height: usize,
    width: usize,
}

impl<'a, T> VecWrapper<'a, T>
where
    T: Default + Copy + Clone,
{
    pub fn wrap(vec: &'a Vec<T>, height: usize, width: usize) -> VecWrapper<'a, T> {
        VecWrapper { vec, height, width }
    }

    pub fn value(&self, row: usize, col: usize) -> T {
        if row < self.height && col < self.width {
            self.vec[row * self.width + col]
        } else {
            T::default()
        }
    }
}
