use core::{ColorType, Size, ImageFilter};

pub trait Texture {
    fn get_id(&self) -> i32;
    fn get_size(&self) -> Size<i32>;
    fn get_color_type(&self) -> ColorType;
    fn get_texture_filter(&self) -> ImageFilter;

    fn export(&self, path: &str);
}