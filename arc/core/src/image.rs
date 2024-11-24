use std::ffi::c_uint;

use crate::{ColorType, ImageFilter, Size};

pub trait Image {
    fn get_id(&self) -> c_uint;
    fn get_size(&self) -> Size<i32>;
    fn get_color_type(&self) -> ColorType;
    fn get_filter(&self) -> ImageFilter;
}
