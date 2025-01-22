use std::rc::Rc;

pub struct TextureCache {
    rect: core::Rect<i32>,
    texture: Rc<dyn crate::Texture>,
}

impl TextureCache {
    pub fn new(rect: core::Rect<i32>, texture: Rc<dyn crate::Texture>) -> Self {
        Self { rect, texture }
    }

    pub fn get_rect(&self) -> core::Rect<i32> {
        self.rect
    }

    pub fn get_texture_rc(&self) -> Rc<dyn crate::Texture> {
        self.texture.clone()
    }
}
