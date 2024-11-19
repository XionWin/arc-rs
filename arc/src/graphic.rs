use std::{borrow::Borrow, rc::Rc};

use crate::{Image, RenderingComponent};

pub struct Graphic {
    renderer: Rc<dyn crate::Renderer>
}


impl Graphic {
    pub fn new(renderer: Rc<dyn crate::Renderer>) -> Self {
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
    fn create_image(&self, size: core::Size<i32>, color_type: core::ColorType, image_filter: core::ImageFilter) -> Box<dyn core::Image> {
        let texture = self.renderer.clone().create_texture(size, color_type, image_filter.into());
        Box::new(Image::new(texture))
    }
    fn load_image_from_file(&self, path: &str, image_filter: core::ImageFilter) -> Box<dyn core::Image> {
        let texture = self.renderer.clone().create_texture_from_file(path, image_filter.into());
        Box::new(Image::new(texture))
    }
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