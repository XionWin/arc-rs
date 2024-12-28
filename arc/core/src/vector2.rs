use crate::Number;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2<T>
where
    T: Number,
{
    x: T,
    y: T,
}

impl<T> Vector2<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> T {
        self.x
    }

    pub fn get_y(&self) -> T {
        self.y
    }
}
