use core::Texture;
use std::{borrow::Borrow, rc::Rc};

#[derive(Debug)]
pub struct TextureCache {
    rectangle: core::Rectangle<i32>,
    margin: core::Margin<i32>,
    texture: Rc<dyn Texture>,
}

impl TextureCache {
    pub fn new(
        rectangle: core::Rectangle<i32>,
        margin: core::Margin<i32>,
        texture: Box<dyn Texture>,
    ) -> Self {
        Self {
            rectangle: rectangle,
            margin,
            texture: texture.into(),
        }
    }

    pub fn get_rectangle(&self) -> core::Rectangle<i32> {
        self.rectangle
    }

    pub fn get_margin(&self) -> core::Margin<i32> {
        self.margin
    }

    pub fn get_texture(&self) -> &dyn core::Texture {
        self.texture.borrow()
    }

    pub fn get_texture_rc(&self) -> Rc<dyn core::Texture> {
        self.texture.clone()
    }
}
