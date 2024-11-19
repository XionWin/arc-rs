use std::{ffi::c_void, rc::Rc};

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
    fn clear_color(&self, color: core::Color) {
        let rgba: core::Rgba = color.into();
        let (r, g, b, a) = rgba.into();
        gl::clear_color(r, g, b, a);
    }
    fn clear(&self) {
        gl::clear(
            crate::ClearBufferMask::COLOR_BUFFER_BIT | crate::ClearBufferMask::DEPTH_BUFFER_BIT,
        );
    }

    fn create_texture(
        self: Rc<Self>,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: arc::TextureFilter,
    ) -> Box<dyn arc::Texture> {
        Box::new(crate::Texture::new(self.clone(), size, color_type, texture_filter))
    }

    fn create_texture_from_file(
        &self,
        path: &str,
        color_type: core::ColorType,
        texture_filter: arc::TextureFilter,
    ) -> Box<dyn arc::Texture> {
        todo!()
    }

    // fn create_texture_with_image_data(&self, image_data: core::ImageData, texture_filter: arc::TextureFilter) -> arc::Texture {
    //     todo!()
    // }

    fn drop_texture(&self, texture: &dyn arc::Texture) {
        gl::delete_texture(texture.get_id() as _);
    }

    fn drop_texture_by_id(&self, texture_id: i32) {
        gl::delete_texture(texture_id as _);
    }
}


impl Drop for GLRenderer {
    fn drop(&mut self) {
        util::print_debug!("gl_renderer droped")
    }
}
