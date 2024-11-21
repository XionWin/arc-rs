use crate::Number;

#[derive(Copy, Clone, Debug)]
pub struct Extent<T>
where T: Number {
    pub x: T,
    pub y: T,
}

impl<T> Extent<T> 
where T: Number {
    pub fn new(x: T, y: T) -> Self {
        Extent { x, y }
    }
}
