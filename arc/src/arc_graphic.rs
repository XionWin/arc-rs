use std::borrow::Borrow;

use crate::{color::Color, Graphic, Image};

pub struct ArcGraphic {
    renderer: Box<dyn crate::Renderer>
}


impl ArcGraphic {
    pub const fn const_new(renderer: Box<dyn crate::Renderer>) -> Self {
        Self {
            renderer
        }
    }
    
    pub fn new(renderer: Box<dyn crate::Renderer>) -> Self {
        Self {
            renderer
        }
    }
}

impl Graphic for ArcGraphic {
    fn get_renderer(&self) -> &dyn crate::Renderer {
        self.renderer.borrow()
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
        let r = self.renderer.create_texture(size, color_type, color_filter);
        Image::new(Box::new(r))
    }
    // fn load_image_from_file(&self, path: &str, color_type: core::ColorType, color_filter: core::TextureFilter) -> Image {
    //     Image::new(Box::new(self.renderer.create_texture_with_file(path, color_type, color_filter)))
    // }
    // fn load_image_data(&self, image_data: ImageData, color_filter: core::TextureFilter) -> Image {
    //     Image::new(Box::new(self.renderer.create_texture_with_image_data(image_data, color_filter)))
    // }
}

impl Drop for ArcGraphic {
    fn drop(&mut self) {
        util::print_debug!("arc_graphic droped")
    }
}