use crate::{Location, Number, Size};

#[derive(Default, Debug, Clone, Copy)]
pub struct Rect<T>
where
    T: Number,
{
    location: Location<T>,
    size: Size<T>,
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

    pub fn get_location(&self) -> &Location<T> {
        &self.location
    }

    pub fn get_size(&self) -> &Size<T> {
        &self.size
    }

    pub fn get_x(&self) -> T {
        self.location.get_x()
    }

    pub fn get_y(&self) -> T {
        self.location.get_y()
    }

    pub fn get_width(&self) -> T {
        self.size.get_width()
    }

    pub fn get_height(&self) -> T {
        self.size.get_height()
    }
}
