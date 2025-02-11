use core::{Rectangle, State, Vertex2};
use std::{borrow::Borrow, fmt::Display};

#[derive(Debug)]
pub struct Primitive {
    _vertexes: Box<[Vertex2]>,
    _state: Box<dyn State>,
    _rectangle: Rectangle<i32>,
}

impl Primitive {
    pub fn new(vertexes: Box<[Vertex2]>, state: Box<dyn State>, rectangle: Rectangle<i32>) -> Self {
        Self {
            _vertexes: vertexes,
            _state: state,
            _rectangle: rectangle,
        }
    }

    pub fn get_vertices(&self) -> &[Vertex2] {
        self._vertexes.borrow()
    }

    pub fn get_state(&self) -> &dyn State {
        self._state.borrow()
    }
    pub fn get_rectangle(&self) -> Rectangle<i32> {
        self._rectangle
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "vertexes: {:#?},\nstate: {:#?}",
            self._vertexes
                .iter()
                .map(|vertex| format!("{}", vertex))
                .collect::<Vec<_>>(),
            self._state
        )
    }
}
