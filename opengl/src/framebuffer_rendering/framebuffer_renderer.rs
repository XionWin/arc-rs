use std::{cell::RefCell, ffi::c_uint, rc::Rc};

use graphic::TextureImage;

use crate::{renderer_utility, AttributeLocation, FragUniform, GLRenderer};

use super::FrameData;

#[derive(Debug)]
pub struct FramebufferRenderer {
    _color_type: core::ColorType,
    _vao: c_uint,
    _vbo: c_uint,
    _fbo: c_uint,
    _multisample_fbo: c_uint,
    _color_multisample_rbo: c_uint,
    // _depth_multisample_rbo: c_uint,
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
        let _multisample_fbo = crate::gl::gen_frame_buffer();
        let _color_multisample_rbo = crate::gl::gen_render_buffer();
        // let _depth_multisample_rbo = crate::gl::gen_render_buffer();
        Self {
            _color_type: core::ColorType::Rgba,
            _vao,
            _vbo,
            _fbo,
            _multisample_fbo,
            _color_multisample_rbo,
            // _depth_multisample_rbo,
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
        let frame_data = &self._frame_data.borrow();
        let frag_uniforms = frame_data.get_frag_uniforms();

        renderer_utility::bind_vertex_array(self._vao);
        // binding vertices
        renderer_utility::bind_data(self, frame_data.get_vertices());

        // [TEST]
        self._program.set_uniform_point_size(5i32);

        // crate::gl::enable(crate::def::EnableCap::Blend);
        // crate::gl::blend_func(
        //     crate::def::BlendingFactorSrc::SrcAlpha,
        //     crate::def::BlendingFactorDest::OneMinusSrcAlpha,
        // );
        crate::gl::disable(crate::def::EnableCap::Blend);

        if crate::get_is_enable_multisample() {
            self.render_with_multisample_framebuffer(frame_data, frag_uniforms);
        } else {
            self.render_with_framebuffer(frame_data, frag_uniforms);
        }

        bind_screen_framebuffer();
    }
}

impl FramebufferRenderer {
    fn render_with_multisample_framebuffer(
        &self,
        frame_data: &FrameData,
        frag_uniforms: &[FragUniform],
    ) {
        bind_multisample_renderbuffer_to_framebuffer(
            self._multisample_fbo,
            self._color_multisample_rbo,
            // self._depth_multisample_rbo,
            core::Size::new(800, 480),
        );
        for call in frame_data.get_calls() {
            let fb_texture = call.get_fb_texture();
            let fb_texture_size = fb_texture.get_size();
            bind_multisample_renderbuffer(self._multisample_fbo);
            self.clear_color(core::Color::Transparent);
            self.clear();
            self.set_rendering_size(fb_texture_size);
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
            bind_texture_to_framebuffer(self._fbo, fb_texture);
            copy_framebuffer(self._multisample_fbo, self._fbo, fb_texture_size);
        }
    }

    fn render_with_framebuffer(&self, frame_data: &FrameData, frag_uniforms: &[FragUniform]) {
        bind_multisample_renderbuffer_to_framebuffer(
            self._multisample_fbo,
            self._color_multisample_rbo,
            // self._depth_multisample_rbo,
            core::Size::new(800, 480),
        );
        for call in frame_data.get_calls() {
            let fb_texture = call.get_fb_texture();
            let fb_texture_size = fb_texture.get_size();
            bind_texture_to_framebuffer(self._fbo, fb_texture);
            self.clear_color(core::Color::Transparent);
            self.clear();
            self.set_rendering_size(fb_texture_size);
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
        bind_texture_to_framebuffer(self._fbo, texture);
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
            .set_uniform_a_viewport(core::Rectangle::new(0, 0, width as _, height as _));
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

fn bind_multisample_renderbuffer_to_framebuffer(
    fbo: c_uint,
    color_multisample_rbo: c_uint,
    // depth_multisample_rbo: c_uint,
    size: core::Size<i32>,
) {
    crate::gl::bind_framebuffer(crate::def::FramebufferTarget::Framebuffer, fbo);

    let samples = crate::get_max_samples();

    crate::gl::bind_renderbuffer(
        crate::def::RenderbufferTarget::Renderbuffer,
        color_multisample_rbo,
    );
    crate::gl::renderbuffer_storage_multisample(
        crate::def::RenderbufferTarget::Renderbuffer,
        samples,
        crate::def::RenderbufferInternalFormat::Rgba8,
        size.get_width(),
        size.get_height(),
    );
    crate::gl::framebuffer_renderbuffer(
        crate::def::FramebufferTarget::Framebuffer,
        crate::def::FramebufferAttachment::ColorAttachment0,
        color_multisample_rbo,
    );

    // crate::gl::bind_renderbuffer(
    //     crate::def::RenderbufferTarget::Renderbuffer,
    //     depth_multisample_rbo,
    // );
    // crate::gl::renderbuffer_storage_multisample(
    //     crate::def::RenderbufferTarget::Renderbuffer,
    //     samples,
    //     crate::def::RenderbufferInternalFormat::DepthComponent24,
    //     size.get_width(),
    //     size.get_height(),
    // );
    // crate::gl::framebuffer_renderbuffer(
    //     crate::def::FramebufferTarget::Framebuffer,
    //     crate::def::FramebufferAttachment::DepthAttachment,
    //     depth_multisample_rbo,
    // );
    check_framebuffer_status();
}

fn bind_multisample_renderbuffer(fbo: c_uint) {
    crate::gl::bind_framebuffer(crate::def::FramebufferTarget::Framebuffer, fbo);

    check_framebuffer_status();
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

    check_framebuffer_status();
}

fn copy_framebuffer(src_fbo: c_uint, dst_fbo: c_uint, size: core::Size<i32>) {
    crate::gl::bind_framebuffer(crate::def::FramebufferTarget::ReadFramebuffer, src_fbo);
    crate::gl::bind_framebuffer(crate::def::FramebufferTarget::DrawFramebuffer, dst_fbo);

    crate::gl::blit_framebuffer(
        0,
        0,
        size.get_width(),
        size.get_height(),
        0,
        0,
        size.get_width(),
        size.get_height(),
        crate::def::ClearBufferMasks::COLOR_BUFFER_BIT,
        crate::def::BlitFramebufferFilter::Nearest,
    );

    check_framebuffer_status();
}

fn bind_screen_framebuffer() {
    crate::gl::bind_framebuffer(crate::def::FramebufferTarget::Framebuffer, 0);
}

fn check_framebuffer_status() {
    match crate::gl::check_framebuffer_status(crate::def::FramebufferTarget::Framebuffer) {
        crate::FramebufferErrorCode::FramebufferComplete => {}
        error_code => {
            util::print_panic!("check_framebuffer_status error: {:?}", error_code);
        }
    }
}
