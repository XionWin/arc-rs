use std::fmt::Display;

use crate::Vector2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex2 {
    pub position: Vector2<f32>,
    pub coorinate: Vector2<f32>,
}

impl Vertex2 {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>, u: impl Into<f32>, v: impl Into<f32>) -> Self {
        Self {
            position: Vector2::new(x.into(), y.into()),
            coorinate: Vector2::new(u.into(), v.into()),
        }
    }
}

impl Display for Vertex2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.position.x, self.position.y, self.coorinate.x, self.coorinate.y
        )
    }
}
