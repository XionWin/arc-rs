use std::{cell::RefCell, rc::Rc};

use graphic::Texture;

use crate::FrameData;

#[derive(Debug)]
pub struct GLRenderer {
    _program: crate::Program,
    _frame_data: RefCell<FrameData>,
    _textures: RefCell<Vec<Rc<dyn graphic::Texture>>>,
}

impl GLRenderer {
    pub fn new<T>(loadfn: T) -> Self
    where
        T: Fn(&str) -> *const std::ffi::c_void,
    {
        crate::load_with(loadfn);
        Self {
            _program: crate::Program::new("resource/shader/arc.vert", "resource/shader/arc.frag"),
            _frame_data: RefCell::new(FrameData::new()),
            _textures: RefCell::new(Vec::new()),
        }
    }
}

impl graphic::Renderer for GLRenderer {
    fn init(&self) {
        self._program.use_program();
    }
    fn begin_render(&self) {}
    fn render(&self) {}
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
        color_type: core::ColorType,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        let texture = Rc::new(crate::Texture::new(
            self.clone(),
            size,
            color_type,
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
        let texture_id = if let Some(paint_image) = state.get_paint().get_paint_image() {
            Some(paint_image.get_image())
        } else {
            None
        };
        self._frame_data.borrow_mut().add_call(
            crate::CallType::Fill,
            primitive.get_vertices(),
            state.into(),
            None,
        );

        util::print_debug_with_title!("frame_data", "{:?}", self._frame_data)
    }
}

impl Drop for GLRenderer {
    fn drop(&mut self) {
        util::print_debug!("gl_renderer droped")
    }
}
