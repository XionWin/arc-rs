use std::borrow::Borrow;

use crate::Element;

#[derive(Debug)]
pub struct Container {
    _rectangle: core::Rectangle<i32>,
    _elements: Vec<Element>,
}

impl Container {
    pub fn new(rectangle: core::Rectangle<i32>) -> Self {
        Self {
            _rectangle: rectangle,
            _elements: Vec::new(),
        }
    }

    pub fn add_container(&mut self, container: Container) {
        self._elements.push(container.into());
    }
    pub fn add_element(&mut self, element: Element) {
        self._elements.push(element);
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

    pub fn get_elements_mut(&mut self) -> Option<Vec<&mut Element>> {
        match self._elements.len() {
            x if x > 0 => Some(
                self._elements
                    .iter_mut()
                    .map(|x| x)
                    .collect::<Vec<&mut Element>>(),
            ),
            _ => None,
        }
    }
}
