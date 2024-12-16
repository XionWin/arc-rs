use std::fmt::Debug;
use std::rc::Rc;

use graphic::{Texture, TextureFilter};

pub trait GLRenderer: Debug {
    fn init(&self);
    fn begin_render(&self);
    fn render(&self);
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: core::Color);
    fn clear(&self);

    fn create_texture(
        self: Rc<Self>,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: TextureFilter,
    ) -> Rc<dyn Texture>;
    fn create_texture_from_file(
        self: Rc<Self>,
        path: &str,
        texture_filter: TextureFilter,
    ) -> Rc<dyn Texture>;
    fn drop_texture(&self, texture: &dyn Texture);
    fn add_primitive(&self, primitive: vector::Primitive);
}
