use std::{ffi::c_uint, fmt::Debug};

use crate::{ColorType, ImageFilter, Size};

pub trait Image: Debug {
    fn get_id(&self) -> c_uint;
    fn get_size(&self) -> Size<i32>;
    fn get_color_type(&self) -> ColorType;
    fn get_filter(&self) -> ImageFilter;
}
