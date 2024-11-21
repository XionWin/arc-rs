use std::{
    cell::Cell,
    fmt::{Display, Write},
    usize, vec,
};

use crate::Number;

#[derive(Debug)]
pub struct Matrix<T>
where
    T: Number,
{
    _row_count: usize,
    _col_count: usize,
    _value: Vec<Cell<T>>,
}

impl<T> Matrix<T>
where
    T: Number,
{
    pub fn new(row_count: usize, col_count: usize) -> Self {
        Self {
            _row_count: row_count,
            _col_count: col_count,
            _value: Self::set_as_identity(
                row_count,
                col_count,
                vec![Cell::new(T::default()); row_count * col_count],
            ),
        }
    }

    pub fn get_row(&self, row_index: usize) -> Vec<&Cell<T>> {
        self._value
            .iter()
            .skip(row_index * self._col_count)
            .take(self._col_count)
            .collect::<Vec<&Cell<T>>>()
    }

    pub fn get_col(&self, col_index: usize) -> Vec<&Cell<T>> {
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

    pub fn get_value(&self) -> Vec<T> {
        self._value.iter().map(|x| x.get()).collect()
    }

    fn set_as_identity(row_count: usize, col_count: usize, values: Vec<Cell<T>>) -> Vec<Cell<T>> {
        let values = values;
        for index in 0..row_count.min(col_count) {
            let cell = &values[index * col_count + index];
            cell.set(T::one());
        }
        values
    }
}

impl<T> Display for Matrix<T>
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
