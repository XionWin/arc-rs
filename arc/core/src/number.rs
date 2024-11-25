use std::fmt::Debug;

pub trait Number:
    Default
    + Copy
    + Clone
    + Debug
    + std::ops::Mul
    + std::ops::AddAssign<<Self as std::ops::Mul>::Output>
{
    fn one() -> Self;
}

impl Number for i32 {
    fn one() -> Self {
        1i32
    }
}
impl Number for f32 {
    fn one() -> Self {
        1f32
    }
}
