use crate::Number;

pub const DEFAULT_EXTENT: Extent<i32> = Extent { x: 0i32, y: 0i32 };

#[derive(Copy, Clone, Debug)]
pub struct Extent<T>
where
    T: Number,
{
    x: T,
    y: T,
}

impl<T> Extent<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Extent { x, y }
    }

    pub fn get_x(&self) -> T {
        self.x
    }

    pub fn get_y(&self) -> T {
        self.y
    }
}

impl<T> Default for Extent<T>
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

impl<T> Into<[f32; 2]> for &Extent<T>
where
    T: Number,
{
    fn into(self) -> [f32; 2] {
        [self.x.into_f32(), self.y.into_f32()]
    }
}
