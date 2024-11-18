use core::Size;
use std::borrow::{Borrow, BorrowMut};

use crate::Texture;

pub struct Image<'a> {
    texture: Box<dyn Texture + 'a>,
}

impl<'a> Image<'a> {
    pub fn new(texture: Box<dyn Texture + 'a>) -> Self {
        Self {
            texture
        }
    }
}

impl<'a> core::Image for Image<'a> {
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

impl<'a> crate::TextureComponent for Image<'a> {
    fn get_texture(&self) -> &dyn Texture {
        self.texture.borrow()
    }

    fn get_texture_mut(&mut self) -> &mut dyn Texture {
        self.texture.borrow_mut()
    }
}