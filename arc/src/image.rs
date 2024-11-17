use core::Texture;
use std::borrow::{Borrow, BorrowMut};

pub struct Image {
    texture: Box<dyn Texture>,
}

impl Image {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Self {
            texture
        }
    }

    pub fn get_texture(&self) -> &dyn Texture {
        self.texture.borrow()
    }

    pub fn get_texture_mut(&mut self) -> &mut dyn Texture {
        self.texture.borrow_mut()
    }

    pub fn get_color_type(&self) -> core::ColorType {
        self.texture.get_color_type()
    }

    pub fn get_texture_filter(&self) -> core::TextureFilter {
        self.texture.get_texture_filter()
    }
}