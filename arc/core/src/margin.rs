use crate::Number;

#[derive(Default, Debug, Clone, Copy)]
pub struct Margin<T>
where
    T: Number,
{
    left: T,
    top: T,
    right: T,
    bottom: T,
}

impl<T> Margin<T>
where
    T: Number,
{
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn get_left(&self) -> T {
        self.left
    }
    pub fn get_top(&self) -> T {
        self.top
    }
    pub fn get_right(&self) -> T {
        self.right
    }
    pub fn get_bottom(&self) -> T {
        self.bottom
    }
}
