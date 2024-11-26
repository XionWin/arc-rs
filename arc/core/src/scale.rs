use crate::Number;

#[derive(Debug, Clone, Copy)]
pub struct Scale<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

impl<T> Scale<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
