use crate::matrix::Matrix;

#[allow(clippy::module_name_repetitions)]
pub struct MatrixSlice<'a, T> {
    rows: usize,
    cols: usize,
    data: &'a [T],
}

impl<'a, T> MatrixSlice<'a, T> {
    #[must_use]
    pub fn new(matrix: &'a Matrix<T>) -> Self {
        Self {
            rows: matrix.rows(),
            cols: matrix.cols(),
            data: matrix.data(),
        }
    }

    /// # Safety
    ///
    /// This function is unsafe (just like [`slice::get_unchecked`])
    ///
    /// For a safe version of this function, see [`get`].
    ///
    /// [`slice::get_unchecked`]: slice::get_unchecked
    /// [`get`]: #method.get
    #[must_use]
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.data.get_unchecked(row * self.cols + col)
    }

    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(unsafe { self.get_unchecked(row, col) })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn rows(&self) -> usize {
        self.rows
    }

    #[must_use]
    pub const fn cols(&self) -> usize {
        self.cols
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.cols)
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.cols).map(
            move |col| self.iter_col(col)
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
        self.data[row * self.cols..(row + 1) * self.cols].iter()
    }

    pub fn iter_col(&self, col: usize) -> impl Iterator<Item = &T> {
        self.data.iter().skip(col).step_by(self.cols)
    }

    #[must_use]
    pub fn clone_buffer(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.data.to_owned()
    }
    
    #[must_use]
    pub const fn data(&self) -> &[T] {
        self.data
    }
}