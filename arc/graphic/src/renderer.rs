use core::{Color, ColorType, Size, Texture, TextureFilter};
use std::fmt::Debug;
use vector::Primitive;

pub trait Renderer: Debug {
    fn init(&self, size: Size<i32>);
    fn begin_render(&self);
    fn render(&self);
    fn get_rendering_size(&self) -> Size<i32>;
    fn set_rendering_size(&self, size: Size<i32>);
    fn clear_color(&self, color: Color);
    fn clear(&self);

    fn create_texture(
        &self,
        size: Size<i32>,
        color_type: ColorType,
        texture_filter: TextureFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn Texture>;
    fn load_texture(
        &self,
        path: &str,
        texture_filter: TextureFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn Texture>;
    fn cache_primitive(&self, primitive: Primitive) -> crate::TextureCache;
    fn update_primitive(&self, primitive: Primitive, cache: &crate::TextureCache);
    fn add_primitive(&self, primitive: Primitive);
    fn export_texture(&self, texture: &dyn Texture, path: &str, color_type: ColorType);
    fn check_gl_error(&self) -> String;
}
