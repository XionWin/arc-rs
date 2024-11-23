use crate::{Matrix, Number, RefVectors};
use std::{
    cell::Cell,
    fmt::{Display, Write},
    usize,
};

#[derive(Debug)]
pub struct MatrixRow<T>
where
    T: Number,
{
    _len: usize,
    _value: Vec<Cell<T>>,
}

impl<T> MatrixRow<T>
where
    T: Number,
{
    pub fn new(vectors: &[T]) -> Self {
        Self {
            _len: vectors.len(),
            _value: vectors
                .iter()
                .map(|x| Cell::new(*x))
                .collect::<Vec<Cell<T>>>(),
        }
    }
    pub fn get_value(&self) -> Vec<&Cell<T>> {
        self._value.iter().map(|x| x).collect()
    }
}

#[derive(Debug)]
pub struct Matrix2D<T>
where
    T: Number,
{
    _row_count: usize,
    _col_count: usize,
    _rows: Vec<MatrixRow<T>>,
}

impl<T> Matrix2D<T>
where
    T: Number,
{
    pub fn new() -> Self {
        Self {
            _row_count: 2,
            _col_count: 3,
            _rows: Self::get_identity_rows(),
        }
    }

    fn get_identity_rows() -> Vec<MatrixRow<T>>
    where
        T: Number,
    {
        vec![
            MatrixRow::new(&[T::one(), T::default(), T::default()]),
            MatrixRow::new(&[T::default(), T::one(), T::default()]),
        ]
    }
}

impl<T> Matrix<T> for Matrix2D<T>
where
    T: Number,
{
    fn get_row_count(&self) -> usize {
        self._row_count
    }

    fn get_col_count(&self) -> usize {
        self._col_count
    }

    fn get_row(&self, row_index: usize) -> RefVectors<'_, T> {
        self._rows[row_index].get_value().into()
    }

    fn get_col(&self, col_index: usize) -> RefVectors<'_, T> {
        self._rows
            .iter()
            .map(|x| x.get_value()[col_index])
            .collect::<Vec<&Cell<T>>>()
            .into()
    }

    fn get_value(&self) -> RefVectors<'_, T> {
        self._rows
            .iter()
            .flat_map(|x| x.get_value())
            .collect::<Vec<&Cell<T>>>()
            .into()
    }

    fn rotate(&self, angle: f32) {
        todo!()
    }
}

impl<T> std::ops::Index<[usize; 2]> for Matrix2D<T>
where
    T: Number,
{
    type Output = Cell<T>;

    fn index(&self, indexes: [usize; 2]) -> &Self::Output {
        let [row_index, col_index] = indexes;
        &self._rows[row_index].get_value()[col_index]
    }
}

impl<T> std::ops::Mul for Matrix2D<T>
where
    T: Number,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            _row_count: 2,
            _col_count: 3,
            _rows: vec![
                MatrixRow::new(&[
                    self.get_row(0) * rhs.get_col(0),
                    self.get_row(0) * rhs.get_col(1),
                    self.get_row(0) * rhs.get_col(2),
                ]),
                MatrixRow::new(&[
                    self.get_row(1) * rhs.get_col(0),
                    self.get_row(1) * rhs.get_col(1),
                    self.get_row(1) * rhs.get_col(2),
                ]),
            ],
        }
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
                str.write_str(&format!("{:?}\t", self._rows[row].get_value()[col].get()))
                    .unwrap();
            }
            str.write_char('\n').unwrap();
        }

        write!(f, "{}", &str)
    }
}
