use core::{Texture, TextureFilter};
use std::fmt::Debug;

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
        is_gen_mipmap: bool,
    ) -> Box<dyn Texture>;
    fn create_texture_from_file(
        &self,
        path: &str,
        texture_filter: TextureFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn Texture>;
    fn cache_primitive(&self, primitive: core::Primitive) -> crate::TextureCache;
    fn update_primitive(&self, primitive: core::Primitive, cache: &crate::TextureCache);
    fn add_primitive(&self, primitive: core::Primitive);
    fn export_texture(&self, texture: &dyn Texture, path: &str, color_type: core::ColorType);
    fn check_gl_error(&self) -> String;
}
