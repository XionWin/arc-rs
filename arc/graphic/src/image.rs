use core::{AsAny, Size};
use std::borrow::Borrow;

use crate::Texture;

#[derive(Debug)]
pub struct Image {
    texture: Box<dyn Texture>,
}

impl Image {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Self { texture }
    }
    fn get_texture(&self) -> &dyn Texture {
        self.texture.borrow()
    }
}

impl AsAny for Image {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn box_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl core::Image for Image {
    fn get_size(&self) -> Size<i32> {
        self.texture.get_size()
    }
    fn get_color_type(&self) -> core::ColorType {
        self.texture.get_color_type()
    }

    fn get_filter(&self) -> core::ImageFilter {
        self.texture.get_texture_filter().into()
    }
    fn get_is_gen_mipmap(&self) -> bool {
        self.texture.get_is_gen_mipmap()
    }
}

pub trait TextureImage {
    fn get_texture(&self) -> &dyn crate::Texture;
}

impl<T: core::Image + ?Sized> TextureImage for T {
    fn get_texture(&self) -> &dyn crate::Texture {
        match self.as_any().downcast_ref::<crate::Image>() {
            Some(image) => image.get_texture(),
            None => util::print_panic!("get texture from _texture_image failed"),
        }
    }
}
