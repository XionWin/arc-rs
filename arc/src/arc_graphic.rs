use core::{ImageData, Texture};

use crate::{color::Color, Graphic};

pub struct ArcGraphic<T>
where T: crate::Renderer {
    renderer: T
}


impl<T> ArcGraphic<T>
where T: crate::Renderer {
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

    fn load_image(&self, path: &str, color_type: core::ColorType, color_filter: core::TextureFilter) {
        self.renderer.load_image(path, color_type, color_filter);
    }
    fn load_image_data(&self, image_data: ImageData, color_filter: core::TextureFilter) {
        self.renderer.load_image_data(image_data, color_filter);
    }

    
    fn drop_texture(&self, texture: &dyn Texture) {
        self.renderer.drop_texture(texture);
    }
}