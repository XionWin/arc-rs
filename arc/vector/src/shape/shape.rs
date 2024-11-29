use std::fmt::Debug;

use crate::{FillState, Primitive, StrokeState};

#[derive(Debug)]
pub struct VectorShape {
    stroke: Primitive,
    fill: Primitive,
}

impl VectorShape {
    pub fn get_stroke(&self) -> &Primitive {
        &self.stroke
    }
    pub fn get_fill(&self) -> &Primitive {
        &self.fill
    }
}

impl<T: core::Shape + ?Sized> VectorObject for T {
    fn get_vector_shape(&self) -> VectorShape {
        VectorShape {
            stroke: Primitive::new(Box::new([]), Box::new(StrokeState::new(1f32))),
            fill: Primitive::new(Box::new([]), Box::new(FillState::default())),
        }
    }
}
