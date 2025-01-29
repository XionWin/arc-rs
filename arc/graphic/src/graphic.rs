use std::{borrow::Borrow, cell::RefCell, ffi::c_uint, rc::Rc};

use crate::{Image, RenderingComponent};

pub struct Graphic {
    renderer: Box<dyn crate::Renderer>,
    _shapes: RefCell<Vec<Rc<RefCell<crate::GraphicShape>>>>,
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
    fn init(&self, size: core::Size<i32>) {
        self.renderer.init(size);
    }
    fn begin_render(&self) {
        self.renderer.begin_render();
        let shapes: &Vec<Rc<RefCell<crate::GraphicShape>>> = &self._shapes.borrow();
        for graphic_shape in shapes {
            let shape: &mut crate::GraphicShape = &mut graphic_shape.as_ref().borrow_mut();
            let fill_primitive = vector::VectorShape::get_fill_primitive(shape.get_shape());
            match fill_primitive {
                Some(fill_primitive) => match shape.get_fill_cache() {
                    Some(_cache) => { /*self.renderer.update_primitive(fill_primitive, _cache) */ }
                    None => {
                        shape.set_fill_cache(Some(self.renderer.cache_primitive(fill_primitive)));
                    }
                },
                None => {}
            }

            let fill_primitive = vector::VectorShape::get_fill_primitive(shape.get_shape());
            match fill_primitive {
                Some(fill_primitive) => {
                    self.renderer.add_primitive(fill_primitive);
                }
                None => {}
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
    fn get_image(&self, texture_id: c_uint) -> Option<Rc<dyn core::Image>> {
        match self.renderer.get_texture(texture_id) {
            Some(texture) => Some(Rc::new(Image::new(texture))),
            None => None,
        }
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
    fn add_shape(&self, shape: Box<dyn core::Shape>) {
        let graphic_shape = Rc::new(RefCell::new(shape.into()));
        self._shapes.borrow_mut().push(graphic_shape);
    }
    fn export_shape_cache(&self) {
        let exe_folder = util::get_exe_path().unwrap();
        let mut index = 0;
        let shapes: &Vec<_> = &self._shapes.borrow();
        for shape in shapes {
            let shape: &RefCell<crate::GraphicShape> = shape.borrow();
            let graphic_shape: &crate::GraphicShape = &shape.borrow();
            match graphic_shape.get_fill_cache() {
                Some(cache) => {
                    self.renderer.export_texture(
                        cache.get_texture(),
                        &format!("{}/cache/{}.png", exe_folder, index),
                        core::ColorType::Rgba,
                    );
                }
                None => {}
            }
            index += 1;
        }
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
