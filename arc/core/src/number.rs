use std::fmt::Debug;

pub trait Number:
    Default
    + Copy
    + Clone
    + Debug
    + std::ops::Mul
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
{
    fn from_value(v: i32) -> Self;
    fn pow_i8(&self, i: i8) -> Self;
    fn into_f32(&self) -> f32;
}

impl Number for i32 {
    fn from_value(value: i32) -> Self {
        value
    }

    fn pow_i8(&self, i: i8) -> Self {
        self.pow(i as _)
    }

    fn into_f32(&self) -> f32 {
        *self as _
    }
}
impl Number for f32 {
    fn from_value(value: i32) -> Self {
        value as f32
    }

    fn pow_i8(&self, i: i8) -> Self {
        self.powi(i as _)
    }

    fn into_f32(&self) -> f32 {
        *self
    }
}
