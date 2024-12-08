use core::Size;
use std::{borrow::Borrow, rc::Rc};

use crate::Texture;

#[derive(Debug)]
pub struct Image {
    texture: Rc<dyn Texture>,
}

impl Image {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }

    // pub fn new_from_file<T>(path: &str, get_texture_func: T) -> Self
    // where
    //     T: Fn(core::Size<i32>, core::ColorType) -> Box<dyn Texture>,
    // {
    //     use core::ImageData;
    //     let image = image::ImageData::new_from_file(path);
    //     let texture = get_texture_func(image.get_size(), image.get_color_type());
    //     Self { texture }
    // }
}

impl core::Image for Image {
    fn get_id(&self) -> std::ffi::c_uint {
        self.texture.get_id()
    }

    fn get_size(&self) -> Size<i32> {
        self.texture.get_size()
    }
    fn get_color_type(&self) -> core::ColorType {
        self.texture.get_color_type()
    }

    fn get_filter(&self) -> core::ImageFilter {
        self.texture.get_texture_filter().into()
    }
}

impl crate::TextureComponent for Image {
    fn get_texture(&self) -> &dyn Texture {
        self.texture.borrow()
    }
}
