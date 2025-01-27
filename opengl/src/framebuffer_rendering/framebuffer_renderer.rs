use std::{cell::RefCell, ffi::c_uint, rc::Rc};

use graphic::TextureImage;

use crate::{renderer_utility, AttributeLocation, GLRenderer};

use super::FrameData;

#[derive(Debug)]
pub struct FramebufferRenderer {
    _color_type: core::ColorType,
    _vao: c_uint,
    _vbo: c_uint,
    _fbo: c_uint,
    _rbo: c_uint,
    _program: crate::FramebufferRenderingProgram,
    _attribute_locations: Box<[AttributeLocation]>,
    _frame_data: RefCell<FrameData>,
}

impl FramebufferRenderer {
    pub fn new() -> Self {
        let _program = crate::FramebufferRenderingProgram::new();
        _program.use_program();
        let _vao = crate::gl::gen_vertex_array();
        let _vbo = crate::gl::gen_buffer();
        let _fbo = crate::gl::gen_frame_buffer();
        let _rbo = crate::gl::gen_render_buffer();
        Self {
            _color_type: core::ColorType::Rgba,
            _vao,
            _vbo,
            _fbo,
            _rbo,
            _program,
            _attribute_locations: Box::new([
                AttributeLocation::new("aPos", 0, 2),
                AttributeLocation::new("aCoord", 2, 2),
            ]),
            _frame_data: RefCell::new(FrameData::new()),
        }
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
        let frame_data = self._frame_data.borrow();
        let frag_uniforms = frame_data.get_frag_uniforms();

        renderer_utility::bind_vertex_array(self._vao);
        // binding vertices
        renderer_utility::bind_data(self, frame_data.get_vertices());

        // [TEST]
        self._program.set_uniform_point_size(5i32);
        crate::gl::disable(crate::def::EnableCap::Blend);

        for call in frame_data.get_calls() {
            let fb_texture = call.get_fb_texture();
            bind_texture_to_framebuffer(self._fbo, self._rbo, fb_texture);
            self.clear_color(core::Color::MagicDeepGray);
            self.clear();
            self.set_rendering_size(fb_texture.get_size());
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
        bind_screen_framebuffer();
    }
}

impl FramebufferRenderer {
    pub fn init(&self, size: core::Size<i32>) {
        self._program.use_program();
        renderer_utility::bind_vertex_array(self._vao);
        self.set_rendering_size(size);
    }
    pub fn add_primitive(&self, primitive: vector::Primitive, texture: Rc<dyn graphic::Texture>) {
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

    pub fn get_rendered_primivitive(&self) -> Vec<vector::Primitive> {
        Vec::new()
    }

    pub fn export_texture(
        &self,
        texture: &dyn graphic::Texture,
        path: &str,
        color_type: core::ColorType,
    ) {
        bind_texture_to_framebuffer(self._fbo, self._rbo, texture);
        let texture_size = texture.get_size();
        let mut buffer = vec![
            0u8;
            (texture_size.get_width()
                * texture_size.get_height()
                * match color_type {
                    core::ColorType::Rgba => 4,
                    core::ColorType::Alpha => 1,
                }) as _
        ];
        crate::gl::read_pixels(
            0,
            0,
            texture_size.get_width(),
            texture_size.get_height(),
            match color_type {
                core::ColorType::Rgba => crate::PixelFormat::Rgba,
                core::ColorType::Alpha => crate::PixelFormat::Alpha,
            },
            crate::PixelType::UnsignedByte,
            &mut buffer,
        );
        image::ImageData::export_to_file(&buffer, texture_size, path);
        bind_screen_framebuffer();
    }

    // pub fn get_rendering_size(&self) -> core::Size<i32> {
    //     self._program.get_rendering_size()
    // }
    pub fn set_rendering_size(&self, size: core::Size<i32>) {
        self._program.use_program();
        let (width, height) = size.into();
        crate::gl::viewport(0, 0, width as _, height as _);
        self._program
            .set_uniform_a_viewport(core::Rect::new(0, 0, width as _, height as _));
    }
    pub fn clear_color(&self, color: core::Color) {
        self._program.use_program();
        let rgba: core::Rgba = color.into();
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

impl Drop for FramebufferRenderer {
    fn drop(&mut self) {
        util::print_debug!("framebuffer_renderer droped")
    }
}

fn bind_texture_to_framebuffer(fbo: c_uint, rbo: c_uint, texture: &dyn graphic::Texture) {
    let texture_size = texture.get_size();
    crate::gl::bind_framebuffer(crate::def::FramebufferTarget::Framebuffer, fbo);
    crate::gl::bind_renderbuffer(crate::def::RenderbufferTarget::Renderbuffer, rbo);

    let max_samples = crate::gl::get_integerv(crate::def::GetPName::MaxSamples);
    crate::gl::renderbuffer_storage_multisample(
        crate::def::RenderbufferTarget::Renderbuffer,
        max_samples.min(4),
        crate::def::RenderbufferInternalFormat::Rgba8,
        texture_size.get_width(),
        texture_size.get_height(),
    );
    crate::gl::framebuffer_renderbuffer(
        crate::def::FramebufferTarget::Framebuffer,
        crate::def::FramebufferAttachment::ColorAttachment0,
        rbo,
    );

    crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, texture.get_id());
    crate::gl::framebuffer_texture_2d(
        crate::def::FramebufferTarget::Framebuffer,
        crate::def::FramebufferAttachment::ColorAttachment0,
        crate::def::TextureTarget2d::Texture2D,
        texture.get_id(),
        0,
    );

    match crate::gl::check_framebuffer_status(crate::def::FramebufferTarget::Framebuffer) {
        crate::FramebufferErrorCode::FramebufferComplete => {}
        _ => util::print_panic!("unexpected"),
    }
}

fn bind_screen_framebuffer() {
    crate::gl::bind_framebuffer(crate::def::FramebufferTarget::Framebuffer, 0);
}
