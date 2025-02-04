use std::ffi::c_uint;

use crate::CallType;

#[derive(Debug)]
pub struct Call {
    rectangle: core::Rectangle<i32>,
    call_type: CallType,
    vertex_offset: usize,
    vertex_len: usize,
    uniform_offset: usize,
    texture_id: Option<c_uint>,
}

impl Call {
    pub fn new(
        rectangle: core::Rectangle<i32>,
        call_type: CallType,
        vertex_offset: usize,
        vertex_len: usize,
        uniform_offset: usize,
        texture_id: Option<c_uint>,
    ) -> Self {
        Self {
            rectangle,
            call_type,
            vertex_offset,
            vertex_len,
            uniform_offset,
            texture_id,
        }
    }

    pub fn get_rectangle(&self) -> core::Rectangle<i32> {
        self.rectangle
    }
    pub fn get_call_type(&self) -> &CallType {
        &self.call_type
    }
    pub fn get_vertex_offset(&self) -> usize {
        self.vertex_offset
    }
    pub fn get_vertex_len(&self) -> usize {
        self.vertex_len
    }
    pub fn get_uniform_offset(&self) -> usize {
        self.uniform_offset
    }
    pub fn get_texture_id(&self) -> Option<c_uint> {
        self.texture_id
    }
}
