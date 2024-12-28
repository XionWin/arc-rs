use crate::Number;

pub trait Pt<T>
where
    T: Number,
{
    fn get_center_point(&self, rhs: &Self) -> Point<f32>;

    fn get_distance(&self, rhs: &Self) -> f32;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T>
where
    T: Number,
{
    x: T,
    y: T,
}

impl<T> Pt<T> for Point<T>
where
    T: Number,
{
    fn get_center_point(&self, rhs: &Self) -> Point<f32> {
        Point {
            x: (self.x + rhs.x).into_f32() / 2f32,
            y: (self.y + rhs.y).into_f32() / 2f32,
        }
    }

    fn get_distance(&self, rhs: &Self) -> f32 {
        ((self.x - rhs.x).pow_i8(2) + (self.y - rhs.y).pow_i8(2))
            .into_f32()
            .sqrt()
    }
}

impl<T> Point<T>
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
