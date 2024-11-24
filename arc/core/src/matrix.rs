use std::cell::Cell;

use crate::Matrix2D;

pub trait Matrix: std::fmt::Debug {
    fn get_row_count(&self) -> usize;
    fn get_col_count(&self) -> usize;
    fn get_row(&self, row_index: usize) -> RefVectors<'_>;
    fn get_col(&self, col_index: usize) -> RefVectors<'_>;

    fn get_value(&self) -> RefVectors<'_>;
    fn rotate(&self, angle: f32) -> Matrix2D;
}

#[derive(Debug)]
pub struct RefVectors<'a> {
    _len: usize,
    _value: Vec<&'a Cell<f32>>,
}

impl<'a> From<Vec<&'a Cell<f32>>> for RefVectors<'a> {
    fn from(value: Vec<&'a Cell<f32>>) -> Self {
        Self {
            _len: value.len(),
            _value: value,
        }
    }
}

impl<'a> std::ops::Index<usize> for RefVectors<'a> {
    type Output = Cell<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self._value[index]
    }
}

impl<'a> std::ops::Mul for RefVectors<'a> {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        let len = self._len.max(rhs._len);
        let mut src = self._value.iter().map(|x| x.get()).collect::<Vec<f32>>();
        for _ in src.len()..len {
            src.push(1f32);
        }
        let mut rhs = rhs._value.iter().map(|x| x.get()).collect::<Vec<f32>>();
        for _ in rhs.len()..len {
            rhs.push(1f32);
        }

        let mut result = 0f32;
        for index in 0..len {
            result += src[index] * rhs[index]
        }
        result
    }
}
