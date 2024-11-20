use crate::Number;

pub struct Matrix<T>
where T: Number {
    _row_count: u32,
    _col_count: u32,
    _value: Vec<Vec<T>>
}

impl<T> Matrix<T>
where T: Number {
    pub fn new(row_count: u32, col_count: u32) -> Self {
        Self {
            _row_count: row_count,
            _col_count: col_count,
            _value: Self::set_as_identity(row_count, col_count, Self::get_default_matrix_data(row_count, col_count))
        }
    }

    fn get_default_matrix_data(row_count: u32, col_count: u32) -> Vec<Vec<T>> {
        let mut result = Vec::new();
        for _ in 0..row_count {
            result.push(vec![T::default(); col_count as usize]);
        }
        result
    }

    fn set_as_identity(row_count: u32, col_count: u32, value: Vec<Vec<T>>) -> Vec<Vec<T>> {
        let mut value = value;
        for row in 0..row_count as usize {
            for col in 0..col_count as usize {
                if row == col {
                    value[row][col] = T::one();
                }
                else {
                    value[row][col] = T::default();
                }
            }
        }
        value
    }
}