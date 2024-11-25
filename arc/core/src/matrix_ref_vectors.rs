use std::{cell::Cell, fmt::Debug};

// pub trait Matrix: Default + Debug {
//     fn get_row_count(&self) -> usize;
//     fn get_col_count(&self) -> usize;
//     fn get_row(&self, row_index: usize) -> RefVectors;
//     fn get_col(&self, col_index: usize) -> RefVectors;

//     fn get_value(&self) -> RefVectors;
//     fn rotate(&self, angle: f32) -> Matrix2D;
// }

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

impl<'a> std::ops::Mul for MatrixRefVectors {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        let max_len = self._len.max(rhs._len);
        let mut src = self._value.iter().map(|x| x.get()).collect::<Vec<f32>>();
        for _ in src.len()..max_len {
            src.push(1f32);
        }
        let mut rhs = rhs._value.iter().map(|x| x.get()).collect::<Vec<f32>>();
        for _ in rhs.len()..max_len {
            rhs.push(1f32);
        }

        let mut result = 0f32;
        for index in 0..max_len {
            result += src[index] * rhs[index]
        }
        result
    }
}
