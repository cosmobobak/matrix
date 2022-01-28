use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    #[must_use] pub fn new(rows: usize, cols: usize) -> Self 
        where T: Default + Clone 
    {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn from_default(rows: usize, cols: usize, default: T) -> Self
        where T: Clone 
    {
        Self {
            rows,
            cols,
            data: vec![default; rows * cols],
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
    #[must_use] pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
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

    #[must_use] pub fn get(&self, row: usize, col: usize) -> Option<&T> {
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

    #[must_use] pub const fn rows(&self) -> usize {
        self.rows
    }

    #[must_use] pub const fn cols(&self) -> usize {
        self.cols
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

    #[must_use] pub fn clone_buffer(&self) -> Vec<T> 
        where T: Clone
    {
        self.data.clone()
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &T {
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.data[row * self.cols + col]
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn global_iteration() {
        let mut m = Matrix::<i32>::new(3, 3);
        m[(0, 0)] = 1;
        m[(1, 1)] = 2;
        m[(2, 2)] = 3;
        assert_eq!(m.iter().sum::<i32>(), 6);
        let vals = m.iter().copied().collect::<Vec<i32>>();
        assert_eq!(vals, &[1, 0, 0, 0, 2, 0, 0, 0, 3]);
    }

    #[test]
    fn row_iteration() {
        let mut m = Matrix::<i32>::new(3, 3);
        m[(0, 0)] = 1;
        m[(1, 1)] = 2;
        m[(2, 2)] = 3;
        assert_eq!(m.iter_row(0).sum::<i32>(), 1);
        assert_eq!(m.iter_row(1).sum::<i32>(), 2);
        assert_eq!(m.iter_row(2).sum::<i32>(), 3);
        let vals = m.iter_row(0).copied().collect::<Vec<i32>>();
        assert_eq!(vals, &[1, 0, 0]);
    }

    #[test]
    fn col_iteration() {
        let mut m = Matrix::<i32>::new(3, 3);
        m[(0, 0)] = 1;
        m[(1, 1)] = 2;
        m[(2, 0)] = 3;
        assert_eq!(m.iter_col(0).sum::<i32>(), 4);
        assert_eq!(m.iter_col(1).sum::<i32>(), 2);
        assert_eq!(m.iter_col(2).sum::<i32>(), 0);
        let vals = m.iter_col(0).copied().collect::<Vec<i32>>();
        assert_eq!(vals, &[1, 0, 3]);
    }

    #[test]
    fn clone_buffer() {
        let mut m = Matrix::<i32>::new(3, 3);
        m[(0, 0)] = 1;
        m[(1, 1)] = 2;
        m[(2, 2)] = 3;
        let vals = m.clone_buffer();
        assert_eq!(vals, &[1, 0, 0, 0, 2, 0, 0, 0, 3]);
    }
}