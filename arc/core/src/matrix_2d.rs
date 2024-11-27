use crate::{matrix_row, MatrixRefVectors, MatrixRow};
use std::{
    cell::Cell,
    fmt::{Display, Write},
    usize,
};

#[derive(Debug)]
pub struct Matrix2D {
    _len: usize,
    _rows: Vec<MatrixRow>,
}

impl Matrix2D {
    pub fn new_from_angle(angle: f32) -> Self {
        Self {
            _len: 6,
            _rows: Self::get_angle_rows(angle),
        }
    }
    pub fn new_from_translation(x: f32, y: f32) -> Self {
        Self {
            _len: 6,
            _rows: Self::get_translation_rows(x, y),
        }
    }

    pub fn get_row_count(&self) -> usize {
        self._rows.len()
    }

    pub fn get_col_count(&self) -> usize {
        self._rows[0].get_len()
    }

    pub fn get_row(&self, row_index: usize) -> MatrixRefVectors {
        self._rows[row_index].get_value().into()
    }

    pub fn get_col(&self, col_index: usize) -> MatrixRefVectors {
        self._rows
            .iter()
            .map(|x| x.get_value()[col_index].clone())
            .collect::<Vec<Cell<f32>>>()
            .into()
    }

    pub fn get_value(&self) -> MatrixRefVectors {
        self._rows
            .iter()
            .flat_map(|x| x.get_value())
            .collect::<Vec<Cell<f32>>>()
            .into()
    }

    pub fn rotate(&self, angle: f32) -> Self {
        self * &Matrix2D::new_from_angle(angle)
    }
}

impl Matrix2D {
    fn get_identity_rows() -> Vec<MatrixRow> {
        vec![
            matrix_row!(1f32, 0f32),
            matrix_row!(0f32, 1f32),
            matrix_row!(0f32, 0f32),
        ]
    }

    fn get_angle_rows(angle: f32) -> Vec<MatrixRow> {
        vec![
            matrix_row!(angle.cos(), -angle.sin()),
            matrix_row!(angle.sin(), angle.cos()),
            matrix_row!(0f32, 0f32),
        ]
    }

    fn get_translation_rows(x: f32, y: f32) -> Vec<MatrixRow> {
        vec![
            matrix_row!(1f32, 0f32),
            matrix_row!(0f32, 1f32),
            matrix_row!(x, y),
        ]
    }
}

impl Default for Matrix2D {
    fn default() -> Self {
        Self {
            _len: 6,
            _rows: Self::get_identity_rows(),
        }
    }
}

impl std::ops::Index<usize> for Matrix2D {
    type Output = MatrixRow;

    fn index(&self, index: usize) -> &Self::Output {
        &self._rows[index]
    }
}

impl std::ops::Mul for &Matrix2D {
    type Output = Matrix2D;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            _len: 6,
            _rows: vec![
                matrix_row!(
                    self[0][0].get() * rhs[0][0].get() + self[0][1].get() * rhs[1][0].get(),
                    self[0][0].get() * rhs[0][1].get() + self[0][1].get() * rhs[1][1].get()
                ),
                matrix_row!(
                    self[1][0].get() * rhs[0][0].get() + self[1][1].get() * rhs[1][0].get(),
                    self[1][0].get() * rhs[0][1].get() + self[1][1].get() * rhs[1][1].get()
                ),
                matrix_row!(
                    self[2][0].get() * rhs[0][0].get()
                        + self[2][1].get() * rhs[1][0].get()
                        + rhs[2][0].get(),
                    self[2][0].get() * rhs[0][1].get()
                        + self[2][1].get() * rhs[1][1].get()
                        + rhs[2][1].get()
                ),
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
                matrix_row!(
                    self[0][0].get() * rhs[0][0].get() + self[0][1].get() * rhs[1][0].get(),
                    self[0][0].get() * rhs[0][1].get() + self[0][1].get() * rhs[1][1].get()
                ),
                matrix_row!(
                    self[1][0].get() * rhs[0][0].get() + self[1][1].get() * rhs[1][0].get(),
                    self[1][0].get() * rhs[0][1].get() + self[1][1].get() * rhs[1][1].get()
                ),
                matrix_row!(
                    self[2][0].get() * rhs[0][0].get()
                        + self[2][1].get() * rhs[1][0].get()
                        + rhs[2][0].get(),
                    self[2][0].get() * rhs[0][1].get()
                        + self[2][1].get() * rhs[1][1].get()
                        + rhs[2][1].get()
                ),
            ],
        }
    }
}

impl Display for Matrix2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for row in &self._rows {
            for cell in &row.get_value() {
                str.write_str(&format!("{:?}\t", cell.get())).unwrap();
            }
            str.write_char('\n').unwrap();
        }

        write!(f, "{}", &str)
    }
}
