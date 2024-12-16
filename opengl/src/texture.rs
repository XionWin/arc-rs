use std::{ffi::c_uint, rc::Rc};

use crate::GLRenderer;

#[derive(Debug)]
pub struct Texture {
    renderer: Rc<dyn GLRenderer>,
    id: c_uint,
    size: core::Size<i32>,
    color_type: core::ColorType,
    texture_filter: graphic::TextureFilter,
}

impl Texture {
    pub fn new(
        renderer: Rc<dyn GLRenderer>,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: graphic::TextureFilter,
    ) -> Self {
        Self {
            renderer,
            id: crate::gl::gen_texture(),
            size,
            color_type,
            texture_filter,
        }
    }
}

impl graphic::Texture for Texture {
    fn get_id(&self) -> c_uint {
        self.id
    }

    fn get_size(&self) -> core::Size<i32> {
        self.size
    }

    fn get_color_type(&self) -> core::ColorType {
        self.color_type
    }

    fn get_texture_filter(&self) -> graphic::TextureFilter {
        self.texture_filter
    }

    fn load(&self, image_data: &dyn core::ImageData) {
        crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, self.id);

        crate::gl::tex_image_2d(
            crate::def::TextureTarget::Texture2D,
            0,
            crate::def::PixelInternalFormat::Rgba,
            image_data.get_size().width,
            image_data.get_size().height,
            0,
            crate::def::PixelFormat::Rgba,
            crate::def::PixelType::UnsignedByte,
            Some(image_data.get_value()),
        );

        let texture_filter = Into::<graphic::TextureFilter>::into(self.texture_filter);
        crate::gl::tex_parameter_i(
            crate::def::TextureTarget::Texture2D,
            crate::def::TextureParameterName::TextureMagFilter,
            Into::<crate::TextureMagFilter>::into(texture_filter) as _,
        );
        crate::gl::tex_parameter_i(
            crate::def::TextureTarget::Texture2D,
            crate::def::TextureParameterName::TextureMinFilter,
            Into::<crate::TextureMinFilter>::into(texture_filter) as _,
        );

        crate::gl::tex_parameter_i(
            crate::def::TextureTarget::Texture2D,
            crate::def::TextureParameterName::TextureWrapS,
            crate::def::TextureWrapMode::Repeat as _,
        );
        crate::gl::tex_parameter_i(
            crate::def::TextureTarget::Texture2D,
            crate::def::TextureParameterName::TextureWrapT,
            crate::def::TextureWrapMode::Repeat as _,
        );

        crate::gl::generate_mipmap(crate::def::TextureTarget::Texture2D);
    }

    fn export(&self, path: &str) {
        println!("path: {}", path);
        todo!()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.renderer.drop_texture(self);
        util::print_debug!("texture {} droped", self.id);
    }
}
