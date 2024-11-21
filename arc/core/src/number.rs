use std::fmt::Debug;

pub trait Number: Copy + Clone + Debug {
    fn default() -> Self;
    fn one() -> Self;
}

impl Number for i32 {
    fn default() -> Self {
        0i32
    }
    fn one() -> Self {
        1i32
    }
}
impl Number for f32 {
    fn default() -> Self {
        0f32
    }
    fn one() -> Self {
        1f32
    }
}