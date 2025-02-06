use std::{borrow::Borrow, rc::Rc};

#[derive(Debug)]
pub struct Container {
    _rectangle: core::Rectangle<i32>,
    _containers: Vec<Rc<Container>>,
}

impl Container {
    pub fn new(rectangle: core::Rectangle<i32>) -> Self {
        Self {
            _rectangle: rectangle,
            _containers: Vec::new(),
        }
    }
    pub fn add_container(&mut self, container: Container) {
        self._containers.push(container.into());
    }
    pub fn get_containers(&self) -> Option<Vec<&Container>> {
        match self._containers.len() {
            x if x > 0 => Some(
                self._containers
                    .iter()
                    .map(|x| Borrow::<Container>::borrow(x))
                    .collect::<Vec<&Container>>(),
            ),
            _ => None,
        }
    }
}
