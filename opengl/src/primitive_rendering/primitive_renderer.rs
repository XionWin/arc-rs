use std::{cell::RefCell, ffi::c_uint};

use super::FrameData;
use crate::{AttributeLocation, GLPrimitiveRenderer, GLRenderer};

#[derive(Debug)]
pub struct PrimitiveRenderer {
    _color_type: core::ColorType,
    _vao: c_uint,
    _vbo: c_uint,
    _vfo: c_uint,
    _program: crate::PrimitiveRenderingProgram,
    _attribute_locations: Box<[AttributeLocation]>,
    _frame_data: RefCell<FrameData>,
}

impl PrimitiveRenderer {
    pub fn new(color_type: core::ColorType) -> Self {
        Self {
            _color_type: color_type,
            _vao: crate::gl::gen_vertex_array(),
            _vbo: crate::gl::gen_buffer(),
            _vfo: crate::gl::gen_frame_buffer(),
            _program: crate::PrimitiveRenderingProgram::new(),
            _attribute_locations: Box::new([AttributeLocation::new("aPos", 0, 2)]),
            _frame_data: RefCell::new(FrameData::new()),
        }
    }
}

impl GLRenderer for PrimitiveRenderer {
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
}

impl GLPrimitiveRenderer for PrimitiveRenderer {
    fn draw_primitive_on_texture(
        &self,
        primitive: vector::Primitive,
        texture: &dyn graphic::Texture,
    ) {
        self._program.use_program();
        self._frame_data.borrow_mut().add_call(
            texture,
            crate::CallType::Fill,
            primitive.get_vertices(),
            crate::DEFAULT_MONOCHROME_FRAG_UNIFORM,
            None,
        );
    }
}

impl Drop for PrimitiveRenderer {
    fn drop(&mut self) {
        util::print_debug!("texture_renderer droped")
    }
}
