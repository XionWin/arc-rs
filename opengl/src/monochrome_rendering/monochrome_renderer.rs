use std::{cell::RefCell, ffi::c_uint};

use graphic::TextureImage;

use crate::{renderer_utility, AttributeLocation, FrameData, GLRenderer, GLTextureRenderer};

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
        let state = primitive.get_state();
        let texture_id = match state.get_paint().try_get_paint_image() {
            Some(paint_image) => Some(paint_image.get_image().get_texture().get_id()),
            None => None,
        };
        self._frame_data.borrow_mut().add_call(
            crate::CallType::Fill,
            primitive.get_vertices(),
            state.into(),
            texture_id,
        );
    }
}

impl GLTextureRenderer for MonochromeRenderer {
    // fn draw_primitive_texture(&self, primitive: &vector::Primitive) -> graphic::TextureCache {
    //     self._program.use_program();
    //     // binding vertices
    //     let vertices = primitive.get_vertices();
    //     renderer_utility::bind_buffer(self, vertices);

    //     self._program.set_viewport(core::Rect::new(0, 0, 800, 480));

    //     crate::gl::disable(crate::def::EnableCap::Blend);
    //     crate::gl::draw_arrays(crate::PrimitiveType::TriangleFan, 0, vertices.len() as _);

    //     graphic::TextureCache::new(
    //         core::Rect::new(0, 0, 100, 100),
    //         Box::new(crate::Texture::new(
    //             core::Size::new(100, 100),
    //             core::ColorType::Alpha,
    //             graphic::TextureFilter::Linear,
    //         )),
    //     )
    // }
}

impl Drop for MonochromeRenderer {
    fn drop(&mut self) {
        util::print_debug!("primitive_renderer droped")
    }
}
