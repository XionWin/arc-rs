use core::{ColorType, Size};
use std::{ffi::c_uint, fmt::Debug};

use crate::TextureFilter;

pub trait Texture: Debug {
    fn get_id(&self) -> c_uint;
    fn get_size(&self) -> Size<i32>;
    fn get_color_type(&self) -> ColorType;
    fn get_texture_filter(&self) -> TextureFilter;
    fn load(&self, image_data: &dyn core::ImageData);
    fn export(&self, path: &str);
}