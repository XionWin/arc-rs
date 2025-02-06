use std::{borrow::Borrow, rc::Rc};

use crate::Element;

#[derive(Debug)]
pub struct Container {
    _rectangle: core::Rectangle<i32>,
    _elements: Vec<Rc<Element>>,
}

impl Container {
    pub fn new(rectangle: core::Rectangle<i32>) -> Self {
        Self {
            _rectangle: rectangle,
            _elements: Vec::new(),
        }
    }

    pub fn add_container(&mut self, container: Container) {
        self._elements.push(Rc::new(container.into()));
    }
    pub fn add_element(&mut self, element: Element) {
        self._elements.push(Rc::new(element));
    }
    pub fn get_elements(&self) -> Option<Vec<&Element>> {
        match self._elements.len() {
            x if x > 0 => Some(
                self._elements
                    .iter()
                    .map(|x| Borrow::<Element>::borrow(x))
                    .collect::<Vec<&Element>>(),
            ),
            _ => None,
        }
    }
}
