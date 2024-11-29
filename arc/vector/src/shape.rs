use std::fmt::Debug;

use crate::{FillState, Primitive, StrokeState};

pub trait VectorObject: Debug {
    fn get_vector_shape(&self) -> VectorShape;
}

#[derive(Debug)]
pub struct VectorShape {
    stroke: Primitive,
    fill: Primitive,
}

impl VectorShape {}

impl<T: core::Shape + ?Sized> VectorObject for T {
    fn get_vector_shape(&self) -> VectorShape {
        VectorShape {
            stroke: Primitive::new(Box::new([]), Box::new(StrokeState::new(1f32))),
            fill: Primitive::new(Box::new([]), Box::new(FillState::default())),
        }
    }
}
