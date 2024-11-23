use std::{borrow::Borrow, cell::Cell};

use crate::Number;

pub trait Matrix<T>: std::fmt::Debug
where
    T: Number,
{
    fn get_row_count(&self) -> usize;
    fn get_col_count(&self) -> usize;
    fn get_row(&self, row_index: usize) -> RefVectors<'_, T>;
    fn get_col(&self, col_index: usize) -> RefVectors<'_, T>;

    fn get_value(&self) -> RefVectors<'_, T>;

    fn rotate(&self, angle: f32);
}

#[derive(Debug)]
pub struct RefVectors<'a, T>
where
    T: Number,
{
    _len: usize,
    _value: Vec<&'a Cell<T>>,
}

impl<'a, T> From<Vec<&'a Cell<T>>> for RefVectors<'a, T>
where
    T: Number,
{
    fn from(value: Vec<&'a Cell<T>>) -> Self {
        Self {
            _len: value.len(),
            _value: value,
        }
    }
}

impl<'a, T> std::ops::Index<usize> for RefVectors<'a, T>
where
    T: Number,
{
    type Output = Cell<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self._value[index]
    }
}

impl<'a, T> std::ops::Mul for RefVectors<'a, T>
where
    T: Number,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        let len = self._len.max(rhs._len);
        let mut src: Vec<T> = self._value.iter().map(|x| x.get()).collect::<Vec<T>>();
        for _ in src.len()..len {
            src.push(T::one());
        }
        let mut rhs = rhs._value.iter().map(|x| x.get()).collect::<Vec<T>>();
        for _ in rhs.len()..len {
            rhs.push(T::one());
        }

        let mut result = T::default();
        for index in 0..len {
            result += src[index] * rhs[index]
        }
        result
    }
}
