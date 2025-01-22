use std::{cell::RefCell, ffi::c_uint, rc::Rc};

use graphic::TextureImage;

use super::FrameData;
use crate::{AttributeLocation, GLRenderer};

#[derive(Debug)]
pub struct FramebufferRenderer {
    _color_type: core::ColorType,
    _vao: c_uint,
    _vbo: c_uint,
    _fbo: c_uint,
    _program: crate::FramebufferRenderingProgram,
    _attribute_locations: Box<[AttributeLocation]>,
    _frame_data: RefCell<FrameData>,
}

impl FramebufferRenderer {
    pub fn new(color_type: core::ColorType) -> Self {
        Self {
            _color_type: color_type,
            _vao: crate::gl::gen_vertex_array(),
            _vbo: crate::gl::gen_buffer(),
            _fbo: crate::gl::gen_frame_buffer(),
            _program: crate::FramebufferRenderingProgram::new(),
            _attribute_locations: Box::new([AttributeLocation::new("aPos", 0, 2)]),
            _frame_data: RefCell::new(FrameData::new()),
        }
    }
    pub fn draw_primitive(&self, primitive: vector::Primitive, texture: Rc<dyn graphic::Texture>) {
        let state = primitive.get_state();
        let texture_id = match state.get_paint().try_get_paint_image() {
            Some(paint_image) => Some(paint_image.get_image().get_texture().get_id()),
            None => None,
        };
        self._frame_data.borrow_mut().add_call(
            texture,
            crate::CallType::Fill,
            primitive.get_vertices(),
            state.into(),
            texture_id,
        );
    }
}

impl GLRenderer for FramebufferRenderer {
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
    fn begin_render(&self) {
        self._frame_data.borrow_mut().reset();
    }
    fn render(&self) {
        self._program.use_program();
        for call in self._frame_data.borrow().get_calls() {
            bind_texture_to_framebuffer(self._fbo, call.get_fb_texture());
        }
    }
}

impl Drop for FramebufferRenderer {
    fn drop(&mut self) {
        util::print_debug!("texture_renderer droped")
    }
}

fn bind_texture_to_framebuffer(fbo: c_uint, texture: &dyn graphic::Texture) {
    crate::gl::bind_framebuffer(crate::def::FramebufferTarget::Framebuffer, fbo);
    crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, texture.get_id());
    crate::gl::framebuffer_texture_2d(
        crate::def::FramebufferTarget::Framebuffer,
        crate::def::FramebufferAttachment::ColorAttachment0,
        crate::def::TextureTarget2d::Texture2D,
        texture.get_id(),
        0,
    );

    if crate::gl::check_framebuffer_status(crate::def::FramebufferTarget::Framebuffer)
        != crate::def::FramebufferErrorCode::FramebufferComplete
    {
        util::print_panic!("unexpected");
    }

    let size = texture.get_size();
    crate::gl::viewport(0, 0, size.get_width(), size.get_height());
}
