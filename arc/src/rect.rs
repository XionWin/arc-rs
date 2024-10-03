use crate::{location::Location, size::Size};

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub location: Location,
    pub size: Size,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            location: Location { x, y },
            size: Size { width, height },
        }
    }
}
