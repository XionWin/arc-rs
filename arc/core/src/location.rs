use crate::Number;

#[derive(Copy, Clone, Debug)]
pub struct Location<T>
where T: Number {
    pub x: T,
    pub y: T,
}

impl<T> Location<T> 
where T: Number {
    pub fn new(x: T, y: T) -> Self {
        Location { x, y }
    }
}
