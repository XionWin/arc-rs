use std::cell::Cell;

use crate::Number;

pub trait Matrix<T>: std::fmt::Debug
where
    T: Number,
{
    fn get_row_count(&self) -> usize;
    fn get_col_count(&self) -> usize;
    fn get_row(&self, row_index: usize) -> Vec<&Cell<T>>;
    fn get_col(&self, col_index: usize) -> Vec<&Cell<T>>;

    fn get_value(&self) -> Vec<&Cell<T>>;
}
