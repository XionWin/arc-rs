use std::fmt::Debug;

use crate::{FillState, Primitive, StrokeState};

pub trait VectorShape: Debug {
    fn get_stroke_primitive(&self) -> Primitive;
    fn get_fill_primitive(&self) -> Primitive;
}

impl<T: core::Shape + ?Sized> VectorShape for T {
    fn get_stroke_primitive(&self) -> Primitive {
        get_stroke_primitive(self.get_commands(), self.get_style())
    }

    fn get_fill_primitive(&self) -> Primitive {
        get_fill_primitive(self.get_commands(), self.get_style())
    }
}

fn get_stroke_primitive(commands: &[core::Command], style: &core::Style) -> Primitive {
    Primitive::new(Box::new([]), Box::new(StrokeState::new(1f32)))
}

fn get_fill_primitive(commands: &[core::Command], style: &core::Style) -> Primitive {
    Primitive::new(Box::new([]), Box::new(FillState::default()))
}
