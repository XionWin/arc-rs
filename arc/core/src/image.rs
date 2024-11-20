use crate::{ColorType, Size, ImageFilter};

pub trait Image {
    fn get_size(&self) -> Size<i32>;
    fn get_color_type(&self) -> ColorType;
    fn get_filter(&self) -> ImageFilter;
}