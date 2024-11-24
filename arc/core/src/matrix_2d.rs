use crate::{Matrix, RefVectors};
use std::{
    cell::Cell,
    fmt::{Display, Write},
    usize,
};

#[derive(Debug)]
pub struct MatrixRow {
    _len: usize,
    _value: Vec<Cell<f32>>,
}

impl MatrixRow {
    pub fn new(vectors: &[f32]) -> Self {
        Self {
            _len: vectors.len(),
            _value: vectors
                .iter()
                .map(|x| Cell::new(*x))
                .collect::<Vec<Cell<f32>>>(),
        }
    }
    pub fn get_value(&self) -> Vec<Cell<f32>> {
        self._value.iter().map(|x| x.clone()).collect()
    }

    pub fn get_ref_value(&self) -> &Vec<Cell<f32>> {
        &self._value
    }
}

#[derive(Debug)]
pub struct Matrix2D {
    _len: usize,
    _rows: Vec<MatrixRow>,
}

impl Matrix2D {
    pub fn new() -> Self {
        Self {
            _len: 6,
            _rows: Self::get_identity_rows(),
        }
    }

    pub fn new_from_angle(angle: f32) -> Self {
        Self {
            _len: 6,
            _rows: Self::get_angle_rows(angle),
        }
    }

    fn get_identity_rows() -> Vec<MatrixRow> {
        vec![
            MatrixRow::new(&[1f32, 0f32, 0f32]),
            MatrixRow::new(&[0f32, 1f32, 0f32]),
        ]
    }

    fn get_angle_rows(angle: f32) -> Vec<MatrixRow> {
        vec![
            MatrixRow::new(&[angle.cos(), -angle.sin(), 0f32]),
            MatrixRow::new(&[angle.sin(), angle.cos(), 0f32]),
        ]
    }
}

impl Matrix for Matrix2D {
    fn get_row_count(&self) -> usize {
        self._rows.len()
    }

    fn get_col_count(&self) -> usize {
        self._rows[0]._len
    }

    fn get_row(&self, row_index: usize) -> RefVectors {
        self._rows[row_index].get_value().into()
    }

    fn get_col(&self, col_index: usize) -> RefVectors {
        self._rows
            .iter()
            .map(|x| x.get_value()[col_index].clone())
            .collect::<Vec<Cell<f32>>>()
            .into()
    }

    fn get_value(&self) -> RefVectors {
        self._rows
            .iter()
            .flat_map(|x| x.get_value())
            .collect::<Vec<Cell<f32>>>()
            .into()
    }

    fn rotate(&self, angle: f32) -> Self {
        self * &Matrix2D::new_from_angle(angle)
    }
}

impl std::ops::Index<usize> for Matrix2D {
    type Output = [Cell<f32>];

    fn index(&self, index: usize) -> &Self::Output {
        self._rows[index].get_ref_value()
    }
}

impl std::ops::Mul for &Matrix2D {
    type Output = Matrix2D;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            _len: 6,
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

impl std::ops::Mul for Matrix2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            _len: 6,
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

impl Display for Matrix2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for row in &self._rows {
            for cell in &row._value {
                str.write_str(&format!("{:?}\t", cell.get())).unwrap();
            }
            str.write_char('\n').unwrap();
        }

        write!(f, "{}", &str)
    }
}
