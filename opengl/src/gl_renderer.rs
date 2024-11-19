use std::rc::Rc;

use arc::Texture;

pub struct GLRenderer {
    _program: Box<dyn arc::Program>,
}

impl GLRenderer {
    pub fn new() -> Self {
        Self {
            _program: Box::new(crate::Program::new(
                "resource/shader/arc.vert",
                "resource/shader/arc.frag",
            )),
        }
    }
}

impl arc::Renderer for GLRenderer {
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
            crate::ClearBufferMask::COLOR_BUFFER_BIT | crate::ClearBufferMask::DEPTH_BUFFER_BIT,
        );
    }

    fn create_texture(
        self: Rc<Self>,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: arc::TextureFilter,
    ) -> Box<dyn arc::Texture> {
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
        texture_filter: arc::TextureFilter,
    ) -> Box<dyn arc::Texture> {
        let image_data = core::ImageData::new_from_file(path);
        let texture = crate::Texture::new(
            self.clone(),
            image_data.size,
            image_data.color_type,
            texture_filter,
        );
        texture.load(&image_data);
        Box::new(texture)
    }

    fn drop_texture(&self, texture: &dyn arc::Texture) {
        crate::gl::delete_texture(texture.get_id());
    }
}

impl Drop for GLRenderer {
    fn drop(&mut self) {
        util::print_debug!("gl_renderer droped")
    }
}
