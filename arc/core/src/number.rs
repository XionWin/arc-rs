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
    + std::cmp::PartialOrd
{
    fn from_i32(v: i32) -> Self;
    fn from_f32(v: f32) -> Self;
    fn into_f32(&self) -> f32;
    fn pow_i8(&self, i: i8) -> Self;
}

impl Number for i32 {
    fn from_i32(value: i32) -> Self {
        value
    }

    fn from_f32(value: f32) -> Self {
        value as _
    }

    fn into_f32(&self) -> f32 {
        *self as _
    }

    fn pow_i8(&self, i: i8) -> Self {
        self.pow(i as _)
    }
}

impl Number for u32 {
    fn from_i32(value: i32) -> Self {
        value as _
    }

    fn from_f32(value: f32) -> Self {
        value as _
    }

    fn into_f32(&self) -> f32 {
        *self as _
    }

    fn pow_i8(&self, i: i8) -> Self {
        self.pow(i as _)
    }
}
impl Number for f32 {
    fn from_i32(value: i32) -> Self {
        value as _
    }

    fn from_f32(value: f32) -> Self {
        value as _
    }

    fn into_f32(&self) -> f32 {
        *self
    }

    fn pow_i8(&self, i: i8) -> Self {
        self.powi(i as _)
    }
}
