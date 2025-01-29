use std::ops::AddAssign;

use crate::{Location, Number, Size};

#[derive(Default, Debug, Clone, Copy)]
pub struct Rectangle<T>
where
    T: Number,
{
    location: Location<T>,
    size: Size<T>,
}

impl<T> Rectangle<T>
where
    T: Number,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            location: Location::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn get_location(&self) -> Location<T> {
        self.location
    }

    pub fn get_size(&self) -> Size<T> {
        self.size
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

impl<T> AddAssign for Rectangle<T>
where
    T: Number,
{
    fn add_assign(&mut self, rhs: Self) {
        let min_x = min(self.location.get_x(), rhs.location.get_x());
        let min_y = min(self.location.get_y(), rhs.location.get_y());

        let max_x = max(
            self.location.get_x() + self.size.get_width(),
            rhs.location.get_x() + rhs.size.get_width(),
        );
        let max_y = max(
            self.location.get_y() + self.size.get_height(),
            rhs.location.get_y() + rhs.size.get_height(),
        );

        self.location = crate::Location::new(min_x, min_y);
        self.size = crate::Size::new(max_x - min_x, max_y - min_y);
    }
}

fn min<T>(lhs: T, rhs: T) -> T
where
    T: Number,
{
    if lhs > rhs {
        rhs
    } else {
        lhs
    }
}
fn max<T>(lhs: T, rhs: T) -> T
where
    T: Number,
{
    if lhs >= rhs {
        lhs
    } else {
        rhs
    }
}
