use std::{borrow::Borrow, cell::RefCell, ffi::c_uint, rc::Rc};

use graphic::Texture;

use crate::{renderer_utility, AttributeLocation, FrameData, GLRenderer};

#[derive(Debug)]
pub struct TextureRenderer {
    _mode: crate::TextureRenderingMode,
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
            _mode: crate::TextureRenderingMode::Alpha,
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
        match self._mode {
            crate::TextureRenderingMode::Alpha => core::ColorType::Alpha,
            crate::TextureRenderingMode::Rgba => core::ColorType::Rgba,
        }
    }
    fn init(&self) {
        self._program.use_program();
        crate::gl::enable_multisample();
        renderer_utility::bind_vertex_array(self._vao);
    }
    fn begin_render(&self) {
        self._frame_data.borrow_mut().reset();
    }
    fn render(&self) {
        let frame_data = self._frame_data.borrow();
        let frag_uniforms = frame_data.get_frag_uniforms();

        renderer_utility::bind_vertex_array(self._vao);
        let vertices = frame_data.get_vertices();
        // binding vertices
        renderer_utility::bind_buffer(self, vertices);

        // [TEST]
        self._program.set_uniform_point_size(5i32);
        self._program.set_viewport(core::Rect::new(0, 0, 800, 480));

        crate::gl::enable(crate::def::EnableCap::Blend);
        crate::gl::blend_func(
            crate::def::BlendingFactorSrc::SrcAlpha,
            crate::def::BlendingFactorDest::OneMinusSrcAlpha,
        );

        for call in frame_data.get_calls() {
            let frag_uniform = frag_uniforms.get(call.uniform_offset).unwrap();
            self._program.set_uniform_frag(frag_uniform);
            if let Some(texture_id) = call.texture_id {
                self._program.set_texture_id(texture_id);
            }

            let primitive_type = match call.call_type {
                crate::CallType::Fill => crate::def::PrimitiveType::TriangleFan,
                crate::CallType::ConvexFill => crate::def::PrimitiveType::TriangleFan,
                crate::CallType::Stroke => crate::def::PrimitiveType::TriangleStrip,
                crate::CallType::Image => crate::def::PrimitiveType::TriangleFan,
            };

            crate::gl::draw_arrays(
                primitive_type,
                call.vertex_offset as _,
                call.vertex_len as _,
            );
        }
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        crate::gl::viewport(x, y, width, height);
    }
    fn clear_color(&self, color: core::Color) {
        let rgba: core::Rgba = color.into();
        let (r, g, b, a) = rgba.into();
        crate::gl::clear_color(r, g, b, a);
    }
    fn clear(&self) {
        crate::gl::clear(
            crate::ClearBufferMasks::COLOR_BUFFER_BIT | crate::ClearBufferMasks::DEPTH_BUFFER_BIT,
        );
    }

    fn create_texture(
        self: Rc<Self>,
        size: core::Size<i32>,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        let texture = Rc::new(crate::Texture::new(
            self.clone(),
            size,
            self.get_color_type(),
            texture_filter,
        ));
        self._textures.borrow_mut().push(texture.clone());
        texture
    }

    fn create_texture_from_file(
        self: Rc<Self>,
        path: &str,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        use core::ImageData;
        let image_data = image::ImageData::new_from_file(path);
        if image_data.get_color_type() != self.get_color_type() {
            util::print_panic!("graphic_renderer create_texture_from_file color_type not matched")
        }
        let texture = Rc::new(crate::Texture::new(
            self.clone(),
            image_data.get_size(),
            image_data.get_color_type(),
            texture_filter,
        ));
        texture.load(&image_data);
        self._textures.borrow_mut().push(texture.clone());
        texture
    }

    fn drop_texture(&self, texture: &dyn graphic::Texture) {
        crate::gl::delete_texture(texture.get_id());
    }

    fn add_primitive(&self, primitive: vector::Primitive) {
        let state = primitive.get_state();
        let texture_id = match state.get_paint().get_paint_image() {
            Some(paint_image) => Some(paint_image.get_image().get_id()),
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

impl Drop for TextureRenderer {
    fn drop(&mut self) {
        util::print_debug!("primitive_renderer droped")
    }
}
