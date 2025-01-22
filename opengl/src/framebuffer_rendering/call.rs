use std::{borrow::Borrow, ffi::c_uint, rc::Rc};

use crate::CallType;

#[derive(Debug)]
pub struct Call {
    fb_texture: Rc<dyn graphic::Texture>,
    call_type: CallType,
    vertex_offset: usize,
    vertex_len: usize,
    uniform_offset: usize,
    texture_id: Option<c_uint>,
}

impl Call {
    pub fn new(
        fb_texture: Rc<dyn graphic::Texture>,
        call_type: CallType,
        vertex_offset: usize,
        vertex_len: usize,
        uniform_offset: usize,
        texture_id: Option<c_uint>,
    ) -> Self {
        Self {
            fb_texture,
            call_type,
            vertex_offset,
            vertex_len,
            uniform_offset,
            texture_id,
        }
    }
    pub fn get_fb_texture(&self) -> &dyn graphic::Texture {
        self.fb_texture.borrow()
    }

    pub fn get_fb_texture_id(&self) -> c_uint {
        self.fb_texture.get_id()
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
