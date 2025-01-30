use std::{borrow::Borrow, rc::Rc};

use super::Container;

#[derive(Debug)]
pub struct PositionContainer {
    _shapes: Vec<Rc<dyn core::Shape>>,
    _rect: Option<core::Rect<i32>>,
}

impl Container for PositionContainer {
    fn add(&mut self, shape: Box<dyn core::Shape>) {
        self.update_rect(shape.get_rect());
        self._shapes.push(shape.into());
    }

    fn get_children(&self) -> Vec<&dyn core::Shape> {
        self._shapes
            .iter()
            .map(|x| Borrow::<dyn core::Shape>::borrow(x))
            .collect::<Vec<&dyn core::Shape>>()
    }

    fn get_rect(&self) -> Option<core::Rect<i32>> {
        self._rect
    }
}

impl PositionContainer {
    pub fn new() -> Self {
        Self {
            _shapes: Vec::new(),
            _rect: None,
        }
    }

    fn update_rect(&mut self, rect: core::Rect<i32>) {
        match &mut self._rect {
            Some(mut self_rect) => self_rect += rect,
            None => self._rect = Some(rect),
        }
    }
}
