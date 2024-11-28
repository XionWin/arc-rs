use std::borrow::Borrow;

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

    pub fn get_vertexes(&self) -> &[core::Vertex2] {
        self._vertexes.borrow()
    }

    pub fn get_state(&self) -> &dyn crate::State {
        self._state.borrow()
    }
}
