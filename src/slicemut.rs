use crate::matrix::Matrix;

pub struct MatrixSliceMut<'a, T> {
    rows: usize,
    cols: usize,
    data: &'a mut [T],
}

impl<'a, T> MatrixSliceMut<'a, T> {
    #[must_use]
    pub fn new(matrix: &'a mut Matrix<T>) -> Self {
        Self {
            rows: matrix.rows(),
            cols: matrix.cols(),
            data: matrix.data_mut(),
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

    /// # Safety
    ///
    /// This function is unsafe (just like [`slice::get_unchecked_mut`])
    ///
    /// For a safe version of this function, see [`get_mut`].
    ///
    /// [`slice::get_unchecked_mut`]: slice::get_unchecked_mut
    /// [`get_mut`]: #method.get_mut
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.data.get_unchecked_mut(row * self.cols + col)
    }

    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(unsafe { self.get_unchecked(row, col) })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(unsafe { self.get_unchecked_mut(row, col) })
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

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_mut(self.cols)
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.cols).map(
            move |col| self.iter_col(col)
        )
    }

    /// # Panics
    /// 
    /// We haven't done this one yet. :)
    #[allow(clippy::unused_self)]
    pub fn iter_cols_mut(&mut self) /* -> impl Iterator<Item = impl Iterator<Item = &mut T>> */ {
        todo!();
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
        self.data[row * self.cols..(row + 1) * self.cols].iter()
    }

    pub fn iter_row_mut(&mut self, row: usize) -> impl Iterator<Item = &mut T> {
        self.data[row * self.cols..(row + 1) * self.cols].iter_mut()
    }

    pub fn iter_col(&self, col: usize) -> impl Iterator<Item = &T> {
        self.data.iter().skip(col).step_by(self.cols)
    }

    pub fn iter_col_mut(&mut self, col: usize) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().skip(col).step_by(self.cols)
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

    #[must_use]
    pub fn data_mut(&mut self) -> &mut [T] {
        self.data
    }
}