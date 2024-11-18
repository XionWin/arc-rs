use std::borrow::Borrow;

use crate::{Image, RenderingComponent};

pub struct Graphic {
    renderer: Box<dyn crate::Renderer>
}


impl Graphic {
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

impl core::Graphic for Graphic {
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.renderer.viewport(x, y, width, height);
    }
    fn clear_color(&self, color: core::Color) {
        self.renderer.clear_color(color);
    }
    fn clear(&self) {
        self.renderer.clear();
    }
    fn create_image(&self, size: core::Size<i32>, color_type: core::ColorType, image_filter: core::ImageFilter) -> Box<dyn core::Image +'_> {
        let r = self.renderer.create_texture(size, color_type, image_filter.into());
        Box::new(Image::new(r))
    }
    // fn load_image_from_file(&self, path: &str, color_type: core::ColorType, image_filter: core::TextureFilter) -> Image {
    //     Image::new(Box::new(self.renderer.create_texture_with_file(path, color_type, image_filter)))
    // }
    // fn load_image_data(&self, image_data: ImageData, image_filter: core::TextureFilter) -> Image {
    //     Image::new(Box::new(self.renderer.create_texture_with_image_data(image_data, image_filter)))
    // }
}

impl RenderingComponent for Graphic {
    fn get_renderer(&self) -> &dyn crate::Renderer {
        self.renderer.borrow()
    }
}

impl Drop for Graphic {
    fn drop(&mut self) {
        util::print_debug!("arc_graphic droped")
    }
}