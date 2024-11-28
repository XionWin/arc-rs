use crate::{Primitive, StrokeState};

pub trait Shape {
    fn get_stroke_primitive(&self) -> Primitive;
    fn get_fill_primitive(&self) -> Primitive;
}

impl<T: core::Shape + ?Sized> Shape for T {
    fn get_stroke_primitive(&self) -> Primitive {
        Primitive::new(Box::new([]), Box::new(StrokeState::new(1f32)))
    }

    fn get_fill_primitive(&self) -> Primitive {
        todo!()
    }
}
