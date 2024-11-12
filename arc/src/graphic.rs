use core::ImageData;

use crate::color::Color;

pub trait Graphic {
    fn viewport(x: i32, y: i32, width: i32, height: i32);
    fn clear_color(color: Color);
    fn clear();

    fn load_image(path: &str, color_type: core::ColorType, color_filter: core::TextureFilter);
    fn load_image_data(image_data: ImageData, color_filter: core::TextureFilter);
}