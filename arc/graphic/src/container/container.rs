use core::AsAny;
use std::{borrow::Borrow, rc::Rc};

use vector::VectorShape;

#[derive(Debug)]
pub struct Container {
    _rectangle: core::Rectangle<i32>,
    _shapes: Vec<Rc<dyn core::Shape>>,
    _containers: Vec<Rc<dyn core::Container>>,
}

impl Container {
    pub fn new(rectangle: core::Rectangle<i32>) -> Self {
        Self {
            _rectangle: rectangle,
            _shapes: Vec::new(),
            _containers: Vec::new(),
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
    fn add_shape(&mut self, shape: Box<dyn core::Shape>) {
        self._shapes.push(shape.into());
    }
    fn add_container(&mut self, container: Box<dyn core::Container>) {
        self._containers.push(container.into());
    }

    fn get_shapes(&self) -> Option<Vec<&dyn core::Shape>> {
        match self._shapes.len() {
            x if x > 0 => Some(
                self._shapes
                    .iter()
                    .map(|x| Borrow::<dyn core::Shape>::borrow(x))
                    .collect::<Vec<&dyn core::Shape>>(),
            ),
            _ => None,
        }
    }
    fn get_containers(&self) -> Option<Vec<&dyn core::Container>> {
        match self._containers.len() {
            x if x > 0 => Some(
                self._containers
                    .iter()
                    .map(|x| Borrow::<dyn core::Container>::borrow(x))
                    .collect::<Vec<&dyn core::Container>>(),
            ),
            _ => None,
        }
    }
}
