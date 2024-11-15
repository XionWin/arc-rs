use std::ffi::c_void;

use crate::gl;

pub struct GLRenderer {}

impl GLRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl GLRenderer {
    pub fn load(&self) {
        gl::load();
    }
    pub fn load_with<T>(&self, loadfn: T)
    where T: Fn(&str) -> *const c_void {
        gl::load_with(loadfn);
    }
}

impl arc::Renderer for GLRenderer {
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        gl::viewport(x, y, width, height);
    }
    fn clear_color(&self, color: arc::Color) {
        let rgba: arc::Rgba = color.into();
        let (r, g, b, a) = rgba.into();
        gl::clear_color(r, g, b, a);
    }
    fn clear(&self) {
        gl::clear(
            crate::ClearBufferMask::COLOR_BUFFER_BIT | crate::ClearBufferMask::DEPTH_BUFFER_BIT,
        );
    }

    fn create_texture(
        &self,
        size: core::Size,
        color_type: core::ColorType,
        color_filter: core::TextureFilter,
    ) -> arc::Texture {
        arc::Texture::new(self)
    }

    fn create_texture_with_file(
        &self,
        path: &str,
        color_type: core::ColorType,
        color_filter: core::TextureFilter,
    ) -> arc::Texture {
        todo!()
    }

    fn create_texture_with_image_data(&self, image_data: core::ImageData, color_filter: core::TextureFilter) -> arc::Texture {
        todo!()
    }

    fn drop_texture(&self, texture: &mut dyn core::Texture) {
        gl::delete_texture(texture.get_id() as _);
    }
}
