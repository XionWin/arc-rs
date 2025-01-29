use core::{AsAny, Size};
use std::{borrow::Borrow, rc::Rc};

use crate::{Texture, TextureComponent};

#[derive(Debug)]
pub struct Image {
    texture: Rc<dyn Texture>,
}

impl Image {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl AsAny for Image {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TextureComponent for Image {
    fn get_texture(&self) -> &dyn Texture {
        self.texture.borrow()
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
            Some(image) => TextureComponent::get_texture(image),
            None => util::print_panic!("get texture from _texture_image failed"),
        }
    }
}
