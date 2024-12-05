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

    pub fn get_center_point(&self, rhs: &Self) -> Point<f32> {
        Point {
            x: (self.x + rhs.x).into_f32() / 2f32,
            y: (self.y + rhs.y).into_f32() / 2f32,
        }
    }

    pub fn get_distance(&self, rhs: &Self) -> f32 {
        ((self.x - rhs.x).pow_i8(2) + (self.y - rhs.y).pow_i8(2))
            .into_f32()
            .sqrt()
    }
}
