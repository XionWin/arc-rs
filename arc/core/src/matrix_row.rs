use std::{cell::Cell, fmt::Display};

#[derive(Debug, PartialEq)]
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

impl Display for MatrixRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self._value
                .iter()
                .map(|x| {
                    let mut number = format!("{:#}", x.get());
                    {
                        let str: String = match number.len() {
                            v if v < 8 => {
                                vec![' '; (8 - number.len()).max(0)].into_iter().collect()
                            }
                            _ => String::new(),
                        };
                        number.push_str(&str);
                        number
                    }
                })
                .collect::<Vec<String>>()
                .join("\t")
        )
    }
}
