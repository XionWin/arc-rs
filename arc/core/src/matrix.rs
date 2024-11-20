use crate::Number;

pub struct Matrix<T>
where T: Number {
    _row_count: u32,
    _col_count: u32,
    _value: Vec<Vec<T>>
}

fn get_default_matrix_data<T>(row_count: u32, col_count: u32) -> Vec<Vec<T>>
where T: Number {
    let mut result = Vec::new();
    for _ in 0..row_count {
        result.push(vec![T::default(); col_count as usize]);
    }
    result
}

impl<T> Matrix<T>
where T: Number {
    pub fn new(row_count: u32, col_count: u32) -> Self {
        Self {
            _row_count: row_count,
            _col_count: col_count,
            _value: get_default_matrix_data(row_count, col_count)
        }
    }
}