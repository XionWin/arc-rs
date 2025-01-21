use std::ffi::c_uint;

use crate::CallType;

#[derive(Debug)]
pub struct Call {
    fb_texture_id: c_uint,
    call_type: CallType,
    vertex_offset: usize,
    vertex_len: usize,
    uniform_offset: usize,
    texture_id: Option<c_uint>,
}

impl Call {
    pub fn new(
        fb_texture_id: c_uint,
        call_type: CallType,
        vertex_offset: usize,
        vertex_len: usize,
        uniform_offset: usize,
        texture_id: Option<c_uint>,
    ) -> Self {
        Self {
            fb_texture_id,
            call_type,
            vertex_offset,
            vertex_len,
            uniform_offset,
            texture_id,
        }
    }

    pub fn get_fb_texture_id(&self) -> c_uint {
        self.fb_texture_id
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
