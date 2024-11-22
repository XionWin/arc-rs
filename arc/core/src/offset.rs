#[derive(Debug)]
pub struct Offset {
    _x: i32,
    _y: i32,
}

impl Offset {
    pub fn new(x: i32, y: i32) -> Self {
        Self { _x: x, _y: y }
    }
}
