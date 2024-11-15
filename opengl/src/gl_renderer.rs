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
        gl::load()
    }
    pub fn load_with<T>(&self, loadfn: T)
    where T: Fn(&str) -> *const c_void {
        gl::load_with(loadfn)
    }
}

impl arc::Renderer for GLRenderer {
    fn load(&self) {
        gl::load();
    }

    fn load_with(&self, load_func: fn(&str) -> *const std::ffi::c_void) {
        gl::load_with(load_func);
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        println!("call viewport function")
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

    fn load_image(
        &self,
        path: &str,
        color_type: core::ColorType,
        color_filter: core::TextureFilter,
    ) {
        todo!()
    }

    fn load_image_data(&self, image_data: core::ImageData, color_filter: core::TextureFilter) {
        todo!()
    }

    fn drop_texture(&self, texture: &dyn core::Texture) {
        todo!()
    }
}
