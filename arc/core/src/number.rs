pub trait Number: Copy + Clone {
    fn default() -> Self;
}

impl Number for i32 {
    fn default() -> Self {
        0i32
    }
}
impl Number for f32 {
    fn default() -> Self {
        0f32
    }}