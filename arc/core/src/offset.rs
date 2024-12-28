use crate::Number;

#[derive(Debug)]
pub struct Offset<T>
where
    T: Number,
{
    x: T,
    y: T,
}

impl<T> Offset<T>
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
