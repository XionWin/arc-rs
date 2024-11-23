use std::{
    cell::Cell,
    fmt::{Display, Write},
    usize,
};

use crate::{Matrix, Number};

#[derive(Debug)]
pub struct Matrix2D<T>
where
    T: Number,
{
    _row_count: usize,
    _col_count: usize,
    _value: [Cell<T>; 6],
}

impl<T> Matrix2D<T>
where
    T: Number,
{
    pub fn new() -> Self {
        Self {
            _row_count: 2,
            _col_count: 3,
            _value: Self::get_identity_matrix(),
        }
    }

    fn get_identity_matrix() -> [Cell<T>; 6]
    where
        T: Number,
    {
        [
            Cell::new(T::one()),
            Cell::new(T::default()),
            Cell::new(T::default()),
            Cell::new(T::default()),
            Cell::new(T::one()),
            Cell::new(T::default()),
        ]
    }
}

impl<T> crate::Matrix<T> for Matrix2D<T>
where
    T: Number,
{
    fn get_row_count(&self) -> usize {
        self._row_count
    }

    fn get_col_count(&self) -> usize {
        self._col_count
    }

    fn get_row(&self, row_index: usize) -> Vec<&Cell<T>> {
        self._value
            .iter()
            .skip(row_index * self._col_count)
            .take(self._col_count)
            .collect::<Vec<&Cell<T>>>()
    }

    fn get_col(&self, col_index: usize) -> Vec<&Cell<T>> {
        let selected_indexes: Vec<usize> = (0..self._row_count)
            .into_iter()
            .map(|r| r * self._col_count + col_index)
            .collect();
        let r = self
            ._value
            .iter()
            .enumerate()
            .filter(|(i, _)| selected_indexes.contains(i))
            .map(|(_, v)| v)
            .collect::<Vec<&Cell<T>>>();
        r
    }

    fn get_value(&self) -> Vec<&Cell<T>> {
        self._value.iter().map(|x| x).collect()
    }
}

impl<T> std::ops::Index<(usize, usize)> for Matrix2D<T>
where
    T: Number,
{
    type Output = Cell<T>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row_index, col_index) = index;
        &self._value[row_index * self.get_col_count() + col_index]
    }
}

impl<T> Display for Matrix2D<T>
where
    T: Number,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for row in 0..self._row_count {
            for col in 0..self._col_count {
                str.write_str(&format!(
                    "{:?}\t",
                    self._value[row * self._col_count + col].get()
                ))
                .unwrap();
            }
            str.write_char('\n').unwrap();
        }

        write!(f, "{}", &str)
    }
}
