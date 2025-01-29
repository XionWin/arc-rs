use std::ops::AddAssign;

use crate::Number;

#[derive(Default, Debug, Clone, Copy)]
pub struct Rect<T>
where
    T: Number,
{
    left: T,
    top: T,
    right: T,
    bottom: T,
}

impl<T> Rect<T>
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

impl<T> AddAssign for Rect<T>
where
    T: Number,
{
    fn add_assign(&mut self, rhs: Self) {
        self.left = min(self.get_left(), rhs.get_left());
        self.top = min(self.get_top(), rhs.get_top());

        self.right = max(self.get_right(), rhs.get_right());
        self.bottom = max(self.get_bottom(), rhs.get_bottom());
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
