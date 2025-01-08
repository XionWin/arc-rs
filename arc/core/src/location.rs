use crate::Number;

#[derive(Default, Copy, Clone, Debug)]
pub struct Location<T>
where
    T: Number,
{
    x: T,
    y: T,
}

impl<T> Location<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Location { x, y }
    }

    pub fn get_x(&self) -> T {
        self.x
    }

    pub fn get_y(&self) -> T {
        self.y
    }
}
