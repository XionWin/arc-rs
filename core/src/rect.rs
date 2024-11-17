use crate::{Location, Size};

pub struct Rect {
    pub location: Location,
    pub size: Size,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            location: Location::new(x, y),
            size: Size::new(width, height)
        }
    }
}