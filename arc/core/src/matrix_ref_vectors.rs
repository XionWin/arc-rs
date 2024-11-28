use std::{cell::Cell, fmt::Debug};

#[derive(Debug)]
pub struct MatrixRefVectors {
    _len: usize,
    _value: Vec<Cell<f32>>,
}

impl From<Vec<Cell<f32>>> for MatrixRefVectors {
    fn from(value: Vec<Cell<f32>>) -> Self {
        Self {
            _len: value.len(),
            _value: value,
        }
    }
}

impl<'a> std::ops::Index<usize> for MatrixRefVectors {
    type Output = Cell<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self._value[index]
    }
}
