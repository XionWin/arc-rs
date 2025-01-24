use std::fmt::Debug;
use std::rc::Rc;

use crate::{Texture, TextureFilter};

pub trait Renderer: Debug {
    fn init(&self, size: core::Size<i32>);
    fn begin_render(&self);
    fn render(&self);
    fn get_rendering_size(&self) -> core::Size<i32>;
    fn set_rendering_size(&self, size: core::Size<i32>);
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
    fn draw_primitive(&self, primitive: vector::Primitive) -> crate::TextureCache;
    fn add_primitive(&self, primitive: vector::Primitive);
    fn export_texture(&self, texture: &dyn crate::Texture, path: &str, color_type: core::ColorType);
}
