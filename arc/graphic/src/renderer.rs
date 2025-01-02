use std::fmt::Debug;
use std::rc::Rc;

use crate::{Texture, TextureFilter};

pub trait Renderer: Debug {
    fn init(&self);
    fn begin_render(&self);
    fn render(&self);
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: core::Color);
    fn clear(&self);

    fn create_texture(
        &self,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: TextureFilter,
    ) -> Rc<dyn Texture>;
    fn create_texture_from_file(
        &self,
        path: &str,
        texture_filter: TextureFilter,
    ) -> Rc<dyn Texture>;
    fn draw_primitive_cache(&self, primitive: vector::Primitive, cache: &crate::TextureCache);
    fn add_primitive(&self, primitive: vector::Primitive);
}
