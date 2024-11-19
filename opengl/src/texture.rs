use std::{ffi::c_uint, rc::Rc};

pub struct Texture {
    renderer: Rc<dyn arc::Renderer>,
    id: c_uint,
    size: core::Size<i32>,
    color_type: core::ColorType,
    texture_filter: arc::TextureFilter,
}

impl Texture {
    pub fn new(
        renderer: Rc<dyn arc::Renderer>,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: arc::TextureFilter,
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

impl arc::Texture for Texture {
    fn get_id(&self) -> c_uint {
        self.id
    }

    fn get_size(&self) -> core::Size<i32> {
        self.size
    }

    fn get_color_type(&self) -> core::ColorType {
        self.color_type
    }

    fn get_texture_filter(&self) -> arc::TextureFilter {
        self.texture_filter
    }

    fn load(&self, image_data: &core::ImageData) {
        crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, self.id);

        crate::gl::tex_image_2d(
            crate::def::TextureTarget::Texture2D,
            0,
            crate::def::PixelInternalFormat::Rgba,
            image_data.size.width,
            image_data.size.height,
            0,
            crate::def::PixelFormat::Rgba,
            crate::def::PixelType::UnsignedByte,
            Some(&image_data.value),
        );

        let texture_filter = Into::<arc::TextureFilter>::into(self.texture_filter);
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

        crate::gl::generate_mipmap(crate::def::GenerateMipmapTarget::Texture2D);
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
