use crate::Number;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
