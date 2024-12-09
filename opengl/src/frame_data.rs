use std::ffi::c_uint;

use crate::{Call, CallType, FragUniform};

#[derive(Debug)]
pub struct FrameData {
    _calls: Vec<Call>,
    _vertices: Vec<core::Vertex2>,
    _frag_uniforms: Vec<FragUniform>,
}

impl FrameData {
    pub fn new() -> Self {
        Self {
            _calls: Vec::new(),
            _vertices: Vec::new(),
            _frag_uniforms: Vec::new(),
        }
    }

    pub fn add_call(
        &mut self,
        call_type: CallType,
        vertices: &[core::Vertex2],
        frag_uniforms: FragUniform,
        texture_id: Option<c_uint>,
    ) {
        let vertex_offset = self._vertices.len();
        let vertex_len = vertices.len();

        for vertex in vertices {
            self._vertices.push(*vertex);
        }

        let uniform_offset = self._frag_uniforms.len();
        self._frag_uniforms.push(frag_uniforms);

        self._calls.push(Call::new(
            call_type,
            vertex_offset,
            vertex_len,
            uniform_offset,
            texture_id,
        ));
    }

    pub fn reset(&mut self) {
        self._calls.clear();
        self._vertices.clear();
        self._frag_uniforms.clear();
    }
}
