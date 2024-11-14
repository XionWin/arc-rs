use core::{ImageData, Texture};

use crate::color::Color;

pub trait Graphic {
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: Color);
    fn clear(&self, );

    fn load_image(&self, path: &str, color_type: core::ColorType, color_filter: core::TextureFilter);
    fn load_image_data(&self, image_data: ImageData, color_filter: core::TextureFilter);

    
    fn drop_texture(&self, texture: &dyn Texture);
}