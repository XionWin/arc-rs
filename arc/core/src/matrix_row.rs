use std::cell::Cell;

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

    pub fn get_len(&self) -> usize {
        self._len
    }

    pub fn get_value(&self) -> Vec<Cell<f32>> {
        self._value.iter().map(|x| x.clone()).collect()
    }
}

impl std::ops::Index<usize> for MatrixRow {
    type Output = Cell<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self._value[index]
    }
}

#[macro_export]
macro_rules! matrix_row {
    ($($ps:expr),+) => {
        crate::MatrixRow::new(&vec![$($ps),+])
    };
}
