use crate::Number;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
