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

    pub fn get_center_point(&self, rhs: &Self) -> Self {
        Self {
            x: (self.x + rhs.x) / T::from_value(2),
            y: (self.y + rhs.y) / T::from_value(2),
        }
    }
}
