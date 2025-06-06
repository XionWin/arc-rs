use crate::{renderer_utility, AttributeLocation, GLRenderer};
use core::{Color, ColorType, Offset, Rectangle, Rgba, Size};
use std::{cell::RefCell, ffi::c_uint};
use vector::Primitive;

use super::FrameData;

#[derive(Debug)]
pub struct GraphicRenderer {
    _color_type: ColorType,
    _vao: c_uint,
    _vbo: c_uint,
    _program: crate::GraphicRenderingProgram,
    _attribute_locations: Box<[AttributeLocation]>,
    _frame_data: RefCell<FrameData>,
}

impl GraphicRenderer {
    pub fn new() -> Self {
        let _program = crate::GraphicRenderingProgram::new();
        _program.use_program();
        let _vao = crate::gl::gen_vertex_array();
        let _vbo = crate::gl::gen_buffer();
        Self {
            _color_type: ColorType::Rgba,
            _vao,
            _vbo,
            _program,
            _attribute_locations: Box::new([
                AttributeLocation::new("aPos", 0, 2),
                AttributeLocation::new("aCoord", 2, 2),
            ]),
            _frame_data: RefCell::new(FrameData::new()),
        }
    }
}

impl GLRenderer for GraphicRenderer {
    fn get_vbo(&self) -> c_uint {
        self._vbo
    }
    fn get_program(&self) -> &dyn crate::GLProgram {
        &self._program
    }
    fn get_attribute_locations(&self) -> &[crate::AttributeLocation] {
        &self._attribute_locations
    }
    fn get_color_type(&self) -> ColorType {
        self._color_type
    }
    fn begin_render(&self) {
        self._frame_data.borrow_mut().reset();
    }
    fn render(&self) {
        self._program.use_program();
        self.set_rendering_size(self.get_rendering_size());
        let frame_data = self._frame_data.borrow();
        let frag_uniforms = frame_data.get_frag_uniforms();

        renderer_utility::bind_vertex_array(self._vao);
        // binding vertices
        renderer_utility::bind_data(self, frame_data.get_vertices());

        crate::gl::enable(crate::def::EnableCap::Blend);
        crate::gl::blend_func(
            crate::def::BlendingFactorSrc::SrcAlpha,
            crate::def::BlendingFactorDest::OneMinusSrcAlpha,
        );

        for call in frame_data.get_calls() {
            self.set_rendering_offset(call.get_rectangle().get_location().into());
            let frag_uniform = frag_uniforms.get(call.get_uniform_offset()).unwrap();
            self._program.set_uniform_frag(frag_uniform);
            if let Some(texture_id) = call.get_texture_id() {
                self._program.use_texture_id(texture_id);
            }

            let primitive_type = match call.get_call_type() {
                crate::CallType::Fill => crate::def::PrimitiveType::TriangleFan,
                crate::CallType::ConvexFill => crate::def::PrimitiveType::TriangleFan,
                crate::CallType::Stroke => crate::def::PrimitiveType::TriangleStrip,
                crate::CallType::Image => crate::def::PrimitiveType::TriangleFan,
            };

            crate::gl::draw_arrays(
                primitive_type,
                call.get_vertex_offset() as _,
                call.get_vertex_len() as _,
            );
        }
    }
}

impl GraphicRenderer {
    pub fn init(&self, size: Size<i32>) {
        self._program.use_program();
        renderer_utility::bind_vertex_array(self._vao);
        self.set_rendering_size(size);
    }
    pub fn add_primitive(&self, primitive: Primitive) {
        let state = primitive.get_state();
        let texture_id = match state.get_paint().try_get_paint_texture() {
            Some(paint_texture) => Some(paint_texture.get_texture().get_id()),
            None => None,
        };
        self._frame_data.borrow_mut().add_call(
            primitive.get_rectangle(),
            crate::CallType::Fill,
            primitive.get_vertices(),
            state.into(),
            texture_id,
        );
    }
    pub fn get_rendering_size(&self) -> Size<i32> {
        self._program.get_rendering_size()
    }
    pub fn set_rendering_size(&self, size: Size<i32>) {
        self._program.use_program();
        let (width, height) = size.into();
        crate::gl::viewport(0, 0, width as _, height as _);
        self._program
            .set_uniform_a_viewport(Rectangle::new(0, 0, width as _, height as _));
    }
    pub fn set_rendering_offset(&self, offset: Offset<i32>) {
        self._program.set_uniform_a_offset(offset);
    }
    pub fn clear_color(&self, color: Color) {
        self._program.use_program();
        let rgba: Rgba = color.into();
        let (r, g, b, a) = rgba.into();
        crate::gl::clear_color(r, g, b, a);
    }
    pub fn clear(&self) {
        self._program.use_program();
        crate::gl::clear(
            crate::ClearBufferMasks::COLOR_BUFFER_BIT | crate::ClearBufferMasks::DEPTH_BUFFER_BIT,
        );
    }
}

impl Drop for GraphicRenderer {
    fn drop(&mut self) {
        util::print_debug!("graphic_renderer droped")
    }
}
