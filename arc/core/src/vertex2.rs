use std::usize;

use crate::Vector2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex2 {
    pub position: Vector2<f32>,
    pub coorinate: Vector2<f32>,
}

impl Vertex2 {
    pub fn new(x: usize, y: usize, u: impl Into<f32>, v: impl Into<f32>) -> Self {
        Self {
            position: Vector2::new(x as _, y as _),
            coorinate: Vector2::new(u.into(), v.into()),
        }
    }
}
