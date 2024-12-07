use std::{borrow::Borrow, fmt::Display};

#[derive(Debug)]
pub struct Primitive {
    _vertexes: Box<[core::Vertex2]>,
    _state: Box<dyn crate::State>,
}

impl Primitive {
    pub fn new(vertexes: Box<[core::Vertex2]>, state: Box<dyn crate::State>) -> Self {
        Self {
            _vertexes: vertexes,
            _state: state,
        }
    }

    pub fn get_vertices(&self) -> &[core::Vertex2] {
        self._vertexes.borrow()
    }

    pub fn get_state(&self) -> &dyn crate::State {
        self._state.borrow()
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
