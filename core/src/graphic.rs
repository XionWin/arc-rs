use crate::{Color, ColorType, Image, Size, ImageFilter};

pub trait Graphic {
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: Color);
    fn clear(&self, );

    fn create_image(&self, size: Size<i32>, color_type: ColorType, image_filter: ImageFilter) -> Box<dyn Image +'_>;
    // fn load_image_from_file(&self, path: &str, color_type: core::ColorType, image_filter: ImageFilter) -> Image;
    // fn load_image_data(&self, image_data: core::ImageData, image_filter: ImageFilter) -> Image;
}