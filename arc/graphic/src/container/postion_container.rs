use std::{borrow::Borrow, rc::Rc};

use super::Container;

#[derive(Debug)]
pub struct PositionContainer {
    _rectangle: core::Rectangle<i32>,
    _shapes: Vec<Rc<dyn core::Shape>>,
}

impl Container for PositionContainer {
    fn add(&mut self, shape: Box<dyn core::Shape>) {
        self._shapes.push(shape.into());
    }

    fn get_children(&self) -> Vec<&dyn core::Shape> {
        self._shapes
            .iter()
            .map(|x| Borrow::<dyn core::Shape>::borrow(x))
            .collect::<Vec<&dyn core::Shape>>()
    }
    fn get_rectangle(&self) -> core::Rectangle<i32> {
        self._rectangle
    }

    fn get_size(&self) -> core::Size<i32> {
        self._rectangle.get_size()
    }

    fn get_rect(&self) -> core::Rect<i32> {
        self._rectangle.into()
    }
}

impl PositionContainer {
    pub fn new(rectangle: core::Rectangle<i32>) -> Self {
        Self {
            _rectangle: rectangle,
            _shapes: Vec::new(),
        }
    }
}
