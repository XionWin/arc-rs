use std::ffi::c_uint;

use crate::CallType;

#[derive(Debug)]
pub struct Call {
    pub target_texture_id: c_uint,
    pub call_type: CallType,
    pub vertex_offset: usize,
    pub vertex_len: usize,
    pub uniform_offset: usize,
    pub texture_id: Option<c_uint>,
}

impl Call {
    pub fn new(
        target_texture_id: c_uint,
        call_type: CallType,
        vertex_offset: usize,
        vertex_len: usize,
        uniform_offset: usize,
        texture_id: Option<c_uint>,
    ) -> Self {
        Self {
            target_texture_id,
            call_type,
            vertex_offset,
            vertex_len,
            uniform_offset,
            texture_id,
        }
    }
}
