use core::{ColorType, Size};

use crate::TextureFilter;

pub trait Texture {
    fn get_id(&self) -> i32;
    fn get_size(&self) -> Size<i32>;
    fn get_color_type(&self) -> ColorType;
    fn get_texture_filter(&self) -> TextureFilter;

    fn export(&self, path: &str);
}