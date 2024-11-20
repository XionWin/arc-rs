use std::ffi::c_uchar;

pub trait ImageData {
    fn get_size(&self) -> crate::Size<i32>;
    fn get_color_type(&self) -> crate::ColorType;
    fn get_value(&self) -> &Vec<c_uchar>;
}