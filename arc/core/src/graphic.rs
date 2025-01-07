use crate::{Color, Image, ImageFilter, Shape, Size};

pub trait Graphic {
    fn get_rendering_size(&self) -> crate::Size<i32>;
    fn init(&self);
    fn begin_render(&self);
    fn render(&self);
    fn set_rendering_size(&mut self, width: i32, height: i32);
    fn clear_color(&self, color: Color);
    fn clear(&self);

    fn create_image(
        &self,
        size: Size<i32>,
        color_type: crate::ColorType,
        image_filter: ImageFilter,
    ) -> Box<dyn Image>;
    fn load_image_from_file(&self, path: &str, image_filter: ImageFilter) -> Box<dyn Image>;
    fn add_shape(&self, shape: Box<dyn Shape>);
}
