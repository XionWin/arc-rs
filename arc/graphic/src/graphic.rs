use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::{Image, RenderingComponent};

pub struct Graphic {
    renderer: Box<dyn crate::Renderer>,
    _shapes: RefCell<Vec<Rc<crate::GraphicShape>>>,
}

impl Graphic {
    pub fn new(renderer: Box<dyn crate::Renderer>) -> Self {
        Self {
            renderer,
            _shapes: RefCell::new(Vec::new()),
        }
    }
}

impl core::Graphic for Graphic {
    fn init(&self) {
        self.renderer.init();
    }
    fn begin_render(&self) {
        self.renderer.begin_render();

        let shapes: &Vec<Rc<crate::GraphicShape>> = &self._shapes.borrow();
        for graphic_shape in shapes {
            let fill_primitive = vector::VectorShape::get_fill_primitive(graphic_shape.get_shape());
            match fill_primitive {
                Some(fill_primitive) => {
                    // util::print_debug!("fill_primitive: {}", fill_primitive);
                    self.renderer.add_primitive(fill_primitive);
                }
                None => {}
            }
        }
    }
    fn render(&self) {
        self.renderer.render();
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.renderer.viewport(x, y, width, height);
    }
    fn clear_color(&self, color: core::Color) {
        self.renderer.clear_color(color);
    }
    fn clear(&self) {
        self.renderer.clear();
    }
    fn create_image(
        &self,
        size: core::Size<i32>,
        color_type: core::ColorType,
        image_filter: core::ImageFilter,
    ) -> Box<dyn core::Image> {
        let texture = self
            .renderer
            .create_texture(size, color_type, image_filter.into());
        Box::new(Image::new(texture))
    }
    fn load_image_from_file(
        &self,
        path: &str,
        image_filter: core::ImageFilter,
    ) -> Box<dyn core::Image> {
        let texture = self
            .renderer
            .create_texture_from_file(path, image_filter.into());
        Box::new(Image::new(texture))
    }
    fn add_shape(&self, shape: Box<dyn core::Shape>) {
        let graphic_shape = Rc::new(shape.into());
        self._shapes.borrow_mut().push(graphic_shape);
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
