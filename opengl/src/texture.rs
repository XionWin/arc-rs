use core::TextureFilter;
use std::ffi::{c_uchar, c_uint};

#[derive(Debug)]
pub struct Texture {
    id: c_uint,
    size: core::Size<i32>,
    color_type: core::ColorType,
    texture_filter: TextureFilter,
    is_gen_mipmap: bool,
}

impl Texture {
    pub fn new(
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: TextureFilter,
        is_gen_mipmap: bool,
    ) -> Self {
        let texture_id = crate::gl::gen_texture();
        create(texture_id, size, texture_filter, is_gen_mipmap);
        Self {
            id: texture_id,
            size,
            color_type,
            texture_filter,
            is_gen_mipmap,
        }
    }

    pub fn load(path: &str, texture_filter: TextureFilter, is_gen_mipmap: bool) -> Self {
        use core::ImageData;
        let image_data = image::ImageData::new_from_file(path);
        let texture_id = crate::gl::gen_texture();
        load(texture_id, &image_data, texture_filter, is_gen_mipmap);
        Self {
            id: texture_id,
            size: image_data.get_size(),
            color_type: image_data.get_color_type(),
            texture_filter,
            is_gen_mipmap,
        }
    }
}

impl core::Texture for Texture {
    fn get_id(&self) -> c_uint {
        self.id
    }

    fn get_size(&self) -> core::Size<i32> {
        self.size
    }

    fn get_color_type(&self) -> core::ColorType {
        self.color_type
    }

    fn get_texture_filter(&self) -> TextureFilter {
        self.texture_filter
    }

    fn get_is_gen_mipmap(&self) -> bool {
        self.is_gen_mipmap
    }
}

fn create(
    texture_id: c_uint,
    size: core::Size<i32>,
    texture_filter: TextureFilter,
    is_gen_mipmap: bool,
) {
    crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, texture_id);

    crate::gl::tex_image_2d::<c_uchar>(
        crate::def::TextureTarget::Texture2D,
        0,
        crate::def::PixelInternalFormat::Rgba,
        size.get_width(),
        size.get_height(),
        0,
        crate::def::PixelFormat::Rgba,
        crate::def::PixelType::UnsignedByte,
        None,
    );

    let texture_filter: TextureFilter = Into::<TextureFilter>::into(texture_filter);
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

    if is_gen_mipmap {
        crate::gl::generate_mipmap(crate::def::TextureTarget::Texture2D);
    }
}

fn load(
    texture_id: c_uint,
    image_data: &dyn core::ImageData,
    texture_filter: TextureFilter,
    is_gen_mipmap: bool,
) {
    crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, texture_id);

    crate::gl::tex_image_2d(
        crate::def::TextureTarget::Texture2D,
        0,
        crate::def::PixelInternalFormat::Rgba,
        image_data.get_size().get_width(),
        image_data.get_size().get_height(),
        0,
        crate::def::PixelFormat::Rgba,
        crate::def::PixelType::UnsignedByte,
        Some(image_data.get_value()),
    );

    let texture_filter = Into::<TextureFilter>::into(texture_filter);
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

    if is_gen_mipmap {
        crate::gl::generate_mipmap(crate::def::TextureTarget::Texture2D);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        crate::gl::delete_texture(self.id);
        util::print_debug!("texture {} droped", self.id);
    }
}
