use std::{borrow::Borrow, cell::RefCell};

use crate::{Element, Image, RenderingComponent};

pub struct Graphic {
    renderer: Box<dyn crate::Renderer>,
    _elements: RefCell<Vec<RefCell<Element>>>,
}

impl Graphic {
    pub fn new(renderer: Box<dyn crate::Renderer>) -> Self {
        Self {
            renderer,
            _elements: RefCell::new(Vec::new()),
        }
    }
}

impl core::Graphic for Graphic {
    fn init(&self, size: core::Size<i32>) {
        self.renderer.init(size);
    }
    fn begin_render(&self) {
        self.renderer.begin_render();
        let shapes: &Vec<RefCell<Element>> = &self._elements.borrow();
        for cell_element in shapes {
            let element = &mut cell_element.borrow_mut();
            if let Some(graphic_shape) = element.get_graphic_shape() {
                let shape = graphic_shape.get_shape();
                let fill_primitive = vector::VectorShape::get_fill_primitive(shape);

                match fill_primitive {
                    Some(fill_primitive) => {
                        self.renderer.add_primitive(fill_primitive);
                    }
                    None => {}
                }
            }

            if element.get_cache().is_none() {
                if let Some(graphic_shape) = element.get_graphic_shape_mut() {
                    let shape = graphic_shape.get_shape_mut();
                    let fill_primitive = vector::VectorShape::get_fill_primitive(shape);
                    // let cache = graphic_shape.get_cache();
                    match fill_primitive {
                        Some(fill_primitive) => {
                            element.set_cache(self.renderer.cache_primitive(fill_primitive));
                        }
                        None => {}
                    }
                }
            }
        }
    }
    fn render(&self) {
        self.renderer.render();
    }
    fn get_rendering_size(&self) -> core::Size<i32> {
        self.renderer.get_rendering_size()
    }
    fn set_rendering_size(&self, size: core::Size<i32>) {
        self.renderer.set_rendering_size(size);
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
        is_gen_mipmap: bool,
    ) -> Box<dyn core::Image> {
        let texture =
            self.renderer
                .create_texture(size, color_type, image_filter.into(), is_gen_mipmap);
        Box::new(Image::new(texture))
    }
    fn load_image_from_file(
        &self,
        path: &str,
        image_filter: core::ImageFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn core::Image> {
        let texture =
            self.renderer
                .create_texture_from_file(path, image_filter.into(), is_gen_mipmap);
        Box::new(Image::new(texture))
    }
    fn add_element(&self, _element: Box<dyn core::Element>) {}
    fn add_shape(&self, shape: Box<dyn core::Shape>) {
        self._elements.borrow_mut().push(RefCell::new(shape.into()));
    }

    fn add_container(&self, container: Box<dyn core::Container>) {
        self._elements
            .borrow_mut()
            .push(RefCell::new(container.into()));
    }
    fn export_shape_cache(&self) {
        let exe_folder = util::get_exe_path().unwrap();
        let mut index = 0;
        let elements: &Vec<_> = &self._elements.borrow();
        for cell_element in elements {
            let element = &cell_element.borrow();
            match element.get_cache() {
                Some(cache) => {
                    self.renderer.export_texture(
                        cache.get_texture(),
                        &format!("{}/cache/{}.png", exe_folder, index),
                        core::ColorType::Rgba,
                    );
                    index += 1;
                }
                None => {}
            }
        }
        util::print_info!("exporting done, total cache count: {}", index);
    }
    fn check_gl_error(&self) -> String {
        self.renderer.check_gl_error()
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
