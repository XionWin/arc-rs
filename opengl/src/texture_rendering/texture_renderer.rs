use std::{cell::RefCell, ffi::c_uint, rc::Rc};

use graphic::Texture;

use crate::{renderer_utility, AttributeLocation, FrameData, GLRenderer};

#[derive(Debug)]
pub struct TextureRenderer {
    _color_type: core::ColorType,
    _vao: c_uint,
    _vbo: c_uint,
    _vfo: c_uint,
    _program: crate::TextureRenderingProgram,
    _attribute_locations: Box<[AttributeLocation]>,
    _frame_data: RefCell<FrameData>,
    _textures: RefCell<Vec<Rc<dyn graphic::Texture>>>,
}

impl TextureRenderer {
    pub fn new() -> Self {
        Self {
            _color_type: core::ColorType::Alpha,
            _vao: crate::gl::gen_vertex_array(),
            _vbo: crate::gl::gen_buffer(),
            _vfo: crate::gl::gen_frame_buffer(),
            _program: crate::TextureRenderingProgram::new(),
            _attribute_locations: Box::new([AttributeLocation::new("aPos", 0, 2)]),
            _frame_data: RefCell::new(FrameData::new()),
            _textures: RefCell::new(Vec::new()),
        }
    }
}

impl GLRenderer for TextureRenderer {
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

impl TextureRenderer {
    pub fn draw_primitive(&self, primitive: vector::Primitive) -> graphic::TextureCache {
        let id = primitive.get_state();
        graphic::TextureCache::new(
            core::Rect::new(0, 0, 100, 100),
            Box::new(crate::Texture::new(
                core::Size::new(100, 100),
                core::ColorType::Alpha,
                graphic::TextureFilter::Linear,
            )),
        )
    }
}

impl Drop for TextureRenderer {
    fn drop(&mut self) {
        util::print_debug!("primitive_renderer droped")
    }
}
