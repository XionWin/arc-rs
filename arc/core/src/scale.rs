use crate::Number;

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
            x: T::from_value(1),
            y: T::from_value(1),
        }
    }
}
