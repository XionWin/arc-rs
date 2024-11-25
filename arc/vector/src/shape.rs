use core::Vector2;

use crate::State;

pub trait Shape {
    fn get_state(&self) -> &dyn State;
    fn get_vertexes(&self) -> &[Vector2<f32>];
}
