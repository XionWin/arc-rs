use core::AsAny;
use std::{borrow::Borrow, rc::Rc};

use vector::VectorShape;

#[derive(Debug)]
pub struct Container {
    _shapes: Vec<Rc<dyn core::Shape>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            _shapes: Vec::new(),
        }
    }
}

impl AsAny for Container {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn box_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl VectorShape for Container {
    fn get_stroke_primitive(&self) -> Option<vector::Primitive> {
        None
    }

    fn get_fill_primitive(&self) -> Option<vector::Primitive> {
        None
    }
}

impl core::Container for Container {
    fn add(&mut self, shape: Box<dyn core::Shape>) {
        self._shapes.push(shape.into());
    }

    fn get_children(&self) -> Vec<&dyn core::Shape> {
        self._shapes
            .iter()
            .map(|x| Borrow::<dyn core::Shape>::borrow(x))
            .collect::<Vec<&dyn core::Shape>>()
    }
}
