use std::fmt::Debug;

pub trait Number:
    Default
    + Copy
    + Clone
    + Debug
    + std::ops::Mul
    + std::ops::Add<Output = Self>
    + std::ops::Div<Output = Self>
{
    fn from_value(v: i32) -> Self;
}

impl Number for i32 {
    fn from_value(value: i32) -> Self {
        value
    }
}
impl Number for f32 {
    fn from_value(value: i32) -> Self {
        value as f32
    }
}
