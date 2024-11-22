use crate::Number;

#[derive(Debug)]
pub struct Offset<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

impl<T> Offset<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
