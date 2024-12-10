use crate::Number;

#[derive(Copy, Clone, Debug)]
pub struct Extent<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

impl<T> Extent<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Extent { x, y }
    }
}

impl<T> Default for Extent<T>
where
    T: Number,
{
    fn default() -> Self {
        Self {
            x: T::from_value(1),
            y: T::from_value(1),
        }
    }
}

impl<T> Into<[f32; 2]> for &Extent<T>
where
    T: Number,
{
    fn into(self) -> [f32; 2] {
        [self.x.into_f32(), self.y.into_f32()]
    }
}
