use crate::{Location, Number, Size};

#[derive(Debug, Clone, Copy)]
pub struct Rect<T>
where
    T: Number,
{
    pub location: Location<T>,
    pub size: Size<T>,
}

impl<T> Rect<T>
where
    T: Number,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            location: Location::new(x, y),
            size: Size::new(width, height),
        }
    }
}
