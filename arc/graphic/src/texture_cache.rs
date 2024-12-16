use std::borrow::Borrow;

pub struct TextureCache {
    rect: core::Rect<i32>,
    texture: Box<dyn crate::Texture>,
}

impl TextureCache {
    pub fn new(rect: core::Rect<i32>, texture: Box<dyn crate::Texture>) -> Self {
        Self { rect, texture }
    }

    pub fn get_rect(&self) -> core::Rect<i32> {
        self.rect
    }

    pub fn get_texture(&self) -> &dyn crate::Texture {
        self.texture.borrow()
    }
}
