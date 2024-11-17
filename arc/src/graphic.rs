use crate::Image;

pub trait Graphic {
    fn get_renderer(&self) -> &dyn crate::Renderer;
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: crate::Color);
    fn clear(&self, );

    fn create_image(&self, size: core::Size<i32>, color_type: core::ColorType, color_filter: core::TextureFilter) -> Image;
    // fn load_image_from_file(&self, path: &str, color_type: core::ColorType, color_filter: core::TextureFilter) -> Image;
    // fn load_image_data(&self, image_data: core::ImageData, color_filter: core::TextureFilter) -> Image;
}