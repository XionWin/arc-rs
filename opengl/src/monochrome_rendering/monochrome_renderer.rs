use std::{cell::RefCell, ffi::c_uint};

use crate::{AttributeLocation, FrameData, GLRenderer};

#[derive(Debug)]
pub struct MonochromeRenderer {
    _color_type: core::ColorType,
    _vao: c_uint,
    _vbo: c_uint,
    _vfo: c_uint,
    _program: crate::MonochromeRenderingProgram,
    _attribute_locations: Box<[AttributeLocation]>,
    _frame_data: RefCell<FrameData>,
}

impl MonochromeRenderer {
    pub fn new() -> Self {
        Self {
            _color_type: core::ColorType::Alpha,
            _vao: crate::gl::gen_vertex_array(),
            _vbo: crate::gl::gen_buffer(),
            _vfo: crate::gl::gen_frame_buffer(),
            _program: crate::MonochromeRenderingProgram::new(),
            _attribute_locations: Box::new([AttributeLocation::new("aPos", 0, 2)]),
            _frame_data: RefCell::new(FrameData::new()),
        }
    }
}

impl GLRenderer for MonochromeRenderer {
    fn get_vbo(&self) -> c_uint {
        self._vbo
    }
    fn get_program(&self) -> &dyn crate::GLProgram {
        &self._program
    }
    fn get_attribute_locations(&self) -> &[crate::AttributeLocation] {
        &self._attribute_locations
    }
    fn get_color_type(&self) -> core::ColorType {
        self._color_type
    }
    fn add_primitive(&self, primitive: vector::Primitive) {
        self._frame_data.borrow_mut().add_call(
            crate::CallType::Fill,
            primitive.get_vertices(),
            crate::DEFAULT_MONOCHROME_FRAG_UNIFORM,
            None,
        );
    }
}

impl Drop for MonochromeRenderer {
    fn drop(&mut self) {
        util::print_debug!("primitive_renderer droped")
    }
}
