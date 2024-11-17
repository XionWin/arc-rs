use crate::Number;

#[derive(Debug, Clone, Copy)]
pub struct Size<T>
where T: Number {
    pub width: T,
    pub height: T,
}

impl<T> Size<T>
where T: Number {
    pub fn new(width: T, height: T) -> Self {
        Self {
            width,
            height
        }
    }
}