#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Location { x, y }
    }
}
