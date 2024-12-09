use crate::{Color, ColorType, Image, ImageFilter, Shape, Size};

pub trait Graphic {
    fn init(&self);
    fn begin_render(&self);
    fn render(&self);
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: Color);
    fn clear(&self);

    fn create_image(
        &self,
        size: Size<i32>,
        color_type: ColorType,
        image_filter: ImageFilter,
    ) -> Box<dyn Image>;
    fn load_image_from_file(&self, path: &str, image_filter: ImageFilter) -> Box<dyn Image>;
    fn draw_shape(&self, shape: &dyn Shape);
}
