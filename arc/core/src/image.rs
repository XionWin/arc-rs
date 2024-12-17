use std::fmt::Debug;

use crate::{AsAny, ColorType, ImageFilter, Size};

pub trait Image: Debug + AsAny {
    fn get_size(&self) -> Size<i32>;
    fn get_color_type(&self) -> ColorType;
    fn get_filter(&self) -> ImageFilter;
}
