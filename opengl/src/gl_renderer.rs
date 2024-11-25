use std::rc::Rc;

use graphic::Texture;

#[derive(Debug)]
pub struct GLRenderer {
    _program: crate::Program,
}

impl GLRenderer {
    pub fn new<T>(loadfn: T) -> Self
    where
        T: Fn(&str) -> *const std::ffi::c_void,
    {
        crate::load_with(loadfn);
        Self {
            _program: crate::Program::new("resource/shader/arc.vert", "resource/shader/arc.frag"),
        }
    }
}

impl graphic::Renderer for GLRenderer {
    fn init(&self) {
        self._program.use_program();
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
        color_type: core::ColorType,
        texture_filter: graphic::TextureFilter,
    ) -> Box<dyn graphic::Texture> {
        Box::new(crate::Texture::new(
            self.clone(),
            size,
            color_type,
            texture_filter,
        ))
    }

    fn create_texture_from_file(
        self: Rc<Self>,
        path: &str,
        texture_filter: graphic::TextureFilter,
    ) -> Box<dyn graphic::Texture> {
        use core::ImageData;
        let image_data = image::ImageData::new_from_file(path);
        let texture = crate::Texture::new(
            self.clone(),
            image_data.get_size(),
            image_data.get_color_type(),
            texture_filter,
        );
        texture.load(&image_data);
        Box::new(texture)
    }

    fn drop_texture(&self, texture: &dyn graphic::Texture) {
        crate::gl::delete_texture(texture.get_id());
    }
}

impl Drop for GLRenderer {
    fn drop(&mut self) {
        util::print_debug!("gl_renderer droped")
    }
}
