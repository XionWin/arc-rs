use std::rc::Rc;

use crate::{Texture, TextureFilter};

pub trait Renderer {
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: core::Color);
    fn clear(&self);

    fn create_texture(
        self: Rc<Self>,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: TextureFilter,
    ) -> Box<dyn Texture>;
    fn create_texture_from_file(
        self: Rc<Self>,
        path: &str,
        texture_filter: TextureFilter,
    ) -> Box<dyn Texture>;
    fn drop_texture(&self, texture: &dyn Texture);
    fn drop_texture_by_id(&self, texture_id: i32);
}
