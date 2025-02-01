use crate::Number;

pub const DEFAULT_SCALE: Scale<f32> = Scale { x: 1f32, y: 1f32 };

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

impl<T> Default for Scale<T>
where
    T: Number,
{
    fn default() -> Self {
        Self {
            x: T::from_i32(1),
            y: T::from_i32(1),
        }
    }
}

impl<T> Into<[f32; 2]> for &Scale<T>
where
    T: Number,
{
    fn into(self) -> [f32; 2] {
        [self.x.into_f32(), self.y.into_f32()]
    }
}
