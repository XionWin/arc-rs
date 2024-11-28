use std::{
    cell::Cell,
    fmt::{Debug, Display},
};

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

impl Display for MatrixRefVectors {
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
