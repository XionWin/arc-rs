use std::fmt::Display;

use crate::Vector2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex2 {
    position: Vector2<f32>,
    coorinate: Vector2<f32>,
}

impl Vertex2 {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>, u: impl Into<f32>, v: impl Into<f32>) -> Self {
        Self {
            position: Vector2::new(x.into(), y.into()),
            coorinate: Vector2::new(u.into(), v.into()),
        }
    }

    pub fn get_x(&self) -> f32 {
        self.position.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.position.get_y()
    }

    pub fn get_u(&self) -> f32 {
        self.coorinate.get_x()
    }

    pub fn get_v(&self) -> f32 {
        self.coorinate.get_y()
    }
}

impl Display for Vertex2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.get_x(),
            self.get_y(),
            self.get_u(),
            self.get_v()
        )
    }
}
