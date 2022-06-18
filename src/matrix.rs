use std::{ops::{Index, IndexMut}, fmt::Display};

use crate::{slice::MatrixSlice, slicemut::MatrixSliceMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    #[must_use]
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    #[must_use]
    pub fn from_default(rows: usize, cols: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            rows,
            cols,
            data: vec![default; rows * cols],
        }
    }

    /// # Panics
    ///
    /// You must provide a vector that has .len() == rows * cols.
    #[must_use]
    pub fn from_parts(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert!(rows * cols == data.len());
        Self {
            rows,
            cols,
            data,
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
        self.data.clone()
    }
    
    #[must_use]
    pub fn data(&self) -> &[T] {
        &self.data
    }

    #[must_use]
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    #[must_use]
    pub fn as_slice(&self) -> MatrixSlice<T> {
        MatrixSlice::new(self)
    }

    #[must_use]
    pub fn as_slice_mut(&mut self) -> MatrixSliceMut<T> {
        MatrixSliceMut::new(self)
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

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.iter_rows().enumerate() {
            if i == 0 {
                write!(f, "[[")?;
            } else {
                write!(f, " [")?;
            }
            for (j, v) in row.iter().enumerate() {
                if j == 0 {
                    write!(f, "{}", v)?;
                } else {
                    write!(f, ", {}", v)?;
                }
            }
            if i == self.rows - 1 {
                writeln!(f, "]]")?;
            } else {
                writeln!(f, "],")?;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! matrix {
    () => {
        {
            // Handle the case when called with no arguments, i.e. matrix![]
            use $crate::matrix::Matrix;
            Matrix::new(0, 0)
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            use $crate::matrix::Matrix;
            let data_as_nested_array = [ $( [ $($x),* ] ),* ];
            let rows = data_as_nested_array.len();
            let cols = data_as_nested_array[0].len();
            let data_as_flat_array: Vec<_> = data_as_nested_array.into_iter()
                .flat_map(|row| row.into_iter())
                .collect();
            Matrix::from_parts(rows, cols, data_as_flat_array)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_simple() {
        let m = matrix![1, 2, 3; 4, 5, 6];
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 3);
        assert_eq!(m.get(0, 0), Some(&1));
        assert_eq!(m.get(0, 1), Some(&2));
        assert_eq!(m.get(0, 2), Some(&3));
        assert_eq!(m.get(1, 0), Some(&4));
        assert_eq!(m.get(1, 1), Some(&5));
        assert_eq!(m.get(1, 2), Some(&6));
        assert_eq!(m.get(2, 0), None);
    }

    #[test]
    fn global_iteration() {
        let m = matrix![
            1, 0, 0;
            0, 2, 0;
            0, 0, 3
        ];
        assert_eq!(m.iter().sum::<i32>(), 6);
        let vals = m.iter().copied().collect::<Vec<i32>>();
        assert_eq!(vals, &[1, 0, 0, 0, 2, 0, 0, 0, 3]);
    }

    #[test]
    fn row_iteration() {
        let m = matrix![
            1, 0, 0;
            0, 2, 0;
            0, 0, 3
        ];
        assert_eq!(m.iter_row(0).sum::<i32>(), 1);
        assert_eq!(m.iter_row(1).sum::<i32>(), 2);
        assert_eq!(m.iter_row(2).sum::<i32>(), 3);
        let vals = m.iter_row(0).copied().collect::<Vec<i32>>();
        assert_eq!(vals, &[1, 0, 0]);
    }

    #[test]
    fn col_iteration() {
        let m = matrix![
            1, 0, 0;
            0, 2, 0;
            3, 0, 0
        ];
        assert_eq!(m.iter_col(0).sum::<i32>(), 4);
        assert_eq!(m.iter_col(1).sum::<i32>(), 2);
        assert_eq!(m.iter_col(2).sum::<i32>(), 0);
        let vals = m.iter_col(0).copied().collect::<Vec<i32>>();
        assert_eq!(vals, &[1, 0, 3]);
    }

    #[test]
    fn clone_buffer() {
        let m = matrix![
            1, 0, 0;
            0, 2, 0;
            0, 0, 3
        ];
        let vals = m.clone_buffer();
        assert_eq!(vals, &[1, 0, 0, 0, 2, 0, 0, 0, 3]);
    }

    #[test]
    fn printing() {
        let m = matrix![
            1, 0, 0;
            0, 2, 0;
            0, 0, 3
        ];
        assert_eq!(format!("{}", m), r#"[[1, 0, 0],
 [0, 2, 0],
 [0, 0, 3]]
"#);
    }
}
