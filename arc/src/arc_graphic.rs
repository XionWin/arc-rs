use core::ImageData;

use crate::{color::Color, Graphic, Image};

pub struct ArcGraphic<T>
where T: crate::Renderer {
    renderer: T
}


impl<T> ArcGraphic<T>
where T: crate::Renderer {
    pub const fn const_new(renderer: T) -> Self {
        Self {
            renderer
        }
    }
    
    pub fn new(renderer: T) -> Self {
        Self {
            renderer
        }
    }
}

impl<T> Graphic for ArcGraphic<T>
where T: crate::Renderer {
    fn get_renderer(&self) -> &dyn crate::Renderer {
        &self.renderer
    }
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.renderer.viewport(x, y, width, height);
    }
    fn clear_color(&self, color: Color) {
        self.renderer.clear_color(color);
    }
    fn clear(&self) {
        self.renderer.clear();
    }
    fn create_image(&self, size: core::Size, color_type: core::ColorType, color_filter: core::TextureFilter) -> Image {
        Image::new(self.renderer.create_texture(size, color_type, color_filter))
    }
    fn load_image_from_file(&self, path: &str, color_type: core::ColorType, color_filter: core::TextureFilter) -> Image {
        todo!()
    }
    fn load_image_data(&self, image_data: ImageData, color_filter: core::TextureFilter) -> Image {
        todo!()
    }
}