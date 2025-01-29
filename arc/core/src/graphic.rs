use std::{ffi::c_uint, rc::Rc};

use crate::{Color, Image, ImageFilter, Shape, Size};

pub trait Graphic {
    fn init(&self, size: crate::Size<i32>);
    fn begin_render(&self);
    fn render(&self);
    fn get_rendering_size(&self) -> crate::Size<i32>;
    fn set_rendering_size(&self, size: crate::Size<i32>);
    fn clear_color(&self, color: Color);
    fn clear(&self);

    fn create_image(
        &self,
        size: Size<i32>,
        color_type: crate::ColorType,
        image_filter: ImageFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn Image>;
    fn load_image_from_file(
        &self,
        path: &str,
        image_filter: ImageFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn Image>;
    fn get_image(&self, texture_id: c_uint) -> Option<Rc<dyn crate::Image>>;
    fn add_shape(&self, shape: Box<dyn Shape>);
    fn export_shape_cache(&self);
    fn check_gl_error(&self) -> String;
}
