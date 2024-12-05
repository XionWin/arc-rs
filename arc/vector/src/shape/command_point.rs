use core::{Number, Point, Pt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CommandPoint<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
    pub is_corner: bool,
}

impl<T> Pt<T> for CommandPoint<T>
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

impl<T> CommandPoint<T>
where
    T: Number,
{
    pub fn new_from_point(point: &Point<T>, is_corner: bool) -> Self {
        Self {
            x: point.x,
            y: point.y,
            is_corner,
        }
    }

    pub fn get_point(&self) -> Point<T> {
        Point::new(self.x, self.y)
    }
}
