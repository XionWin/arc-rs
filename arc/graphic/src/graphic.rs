use std::{cell::RefCell, rc::Rc};

use crate::{Container, Element};

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
    pub fn init(&self, size: core::Size<i32>) {
        self.renderer.init(size);
    }
    pub fn begin_render(&self) {
        self.renderer.begin_render();
        let shapes: &Vec<RefCell<Element>> = &self._elements.borrow();
        for cell_element in shapes {
            let element = &mut cell_element.borrow_mut();

            // if let Some(graphic_shape) = element.get_graphic_shape() {
            //     let shape = graphic_shape.get_shape();
            //     let fill_primitive = vector::VectorShape::get_fill_primitive(shape);

            //     match fill_primitive {
            //         Some(fill_primitive) => {
            //             self.renderer.add_primitive(fill_primitive);
            //         }
            //         None => {}
            //     }
            // }

            recurse_element_mut(element, &mut |element| {
                begin_render_cached_element(self, element);
                if let Some(cache) = element.get_cache() {
                    self.renderer.add_primitive(get_cache_primitive(cache));
                }
            });
        }
    }
    pub fn render(&self) {
        self.renderer.render();
    }
    pub fn get_rendering_size(&self) -> core::Size<i32> {
        self.renderer.get_rendering_size()
    }
    pub fn set_rendering_size(&self, size: core::Size<i32>) {
        self.renderer.set_rendering_size(size);
    }
    pub fn clear_color(&self, color: core::Color) {
        self.renderer.clear_color(color);
    }
    pub fn clear(&self) {
        self.renderer.clear();
    }
    pub fn create_texture(
        &self,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: core::TextureFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn core::Texture> {
        let texture =
            self.renderer
                .create_texture(size, color_type, texture_filter.into(), is_gen_mipmap);
        texture
    }
    pub fn load_texture_from_file(
        &self,
        path: &str,
        texture_filter: core::TextureFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn core::Texture> {
        let texture =
            self.renderer
                .create_texture_from_file(path, texture_filter.into(), is_gen_mipmap);
        texture
    }
    pub fn add_shape(&self, shape: Box<dyn core::Shape>) {
        self._elements.borrow_mut().push(RefCell::new(shape.into()));
    }

    pub fn add_container(&self, container: Container) {
        self._elements
            .borrow_mut()
            .push(RefCell::new(container.into()));
    }
    pub fn export_shape_cache(&self) {
        let exe_folder = util::get_exe_path().unwrap();
        let elements: &Vec<_> = &self._elements.borrow();

        for cell in elements {
            let element = &cell.borrow();
            recurse_element(element, &mut |element| {
                export_element_cache(self, element, &exe_folder)
            });
        }

        util::print_info!("exporting done");
    }

    pub fn check_gl_error(&self) -> String {
        self.renderer.check_gl_error()
    }
}

impl Drop for Graphic {
    fn drop(&mut self) {
        util::print_debug!("arc_graphic droped")
    }
}

fn recurse_element<T>(element: &crate::Element, func: &mut T)
where
    T: FnMut(&crate::Element),
{
    // into next level
    if let Some(container) = element.get_container() {
        if let Some(elements) = container.get_elements() {
            for element in elements {
                recurse_element(element, func);
            }
        }
    }
    // execute func for current element
    func(element);
}

fn recurse_element_mut<T>(element: &mut crate::Element, func: &mut T)
where
    T: FnMut(&mut crate::Element),
{
    // into next level
    if let Some(container) = element.get_container_mut() {
        if let Some(elements) = container.get_elements_mut() {
            for element in elements {
                recurse_element_mut(element, func);
            }
        }
    }
    // execute func for current element
    func(element);
}

fn begin_render_cached_element(g: &Graphic, element: &mut crate::Element) {
    if element.get_cache().is_none() {
        if let Some(graphic_shape) = element.get_graphic_shape_mut() {
            let shape = graphic_shape.get_shape_mut();
            let fill_primitive = vector::VectorShape::get_fill_primitive(shape);
            // let cache = graphic_shape.get_cache();
            match fill_primitive {
                Some(fill_primitive) => {
                    element.set_cache(g.renderer.cache_primitive(fill_primitive));
                }
                None => {}
            }
        }
    }
}

// fn begin_render_children_element(g: &Graphic, element: &mut crate::Element) {
//     if element.get_cache().is_none() {
//         if let Some(graphic_shape) = element.get_graphic_shape_mut() {
//             let shape = graphic_shape.get_shape_mut();
//             let fill_primitive = vector::VectorShape::get_fill_primitive(shape);
//             // let cache = graphic_shape.get_cache();
//             match fill_primitive {
//                 Some(fill_primitive) => {
//                     element.set_cache(g.renderer.cache_primitive(fill_primitive));
//                 }
//                 None => {}
//             }
//         }
//     }
// }

fn get_cache_primitive(cache: &crate::TextureCache) -> core::Primitive {
    let rectangle: core::Rectangle<i32> = cache.get_rectangle();
    let vertexes = Box::new([
        core::Vertex2::new(0f32, 0f32, 0.5f32, 1f32),
        core::Vertex2::new(0f32 as f32, rectangle.get_height() as f32, 0.5f32, 1f32),
        core::Vertex2::new(
            rectangle.get_width() as f32,
            rectangle.get_height() as f32,
            0.5f32,
            1f32,
        ),
        core::Vertex2::new(rectangle.get_width() as f32, 0f32, 0.5f32, 1f32),
    ]);

    let style: core::Style = core::Style::new(
        Box::new(core::TextureBackground::new(Rc::new(
            core::PaintTexture::new(cache.get_texture_rc(), rectangle),
        ))),
        core::ColorBackground::new(core::Color::MoselleGreen, core::Color::MoselleGreen),
        Some(1i32),
    );

    // let style: core::Style = core::Style::new(
    //     Box::new(core::ColorBackground::new(
    //         core::Color::MoselleGreen,
    //         core::Color::MidnightBlue,
    //     )),
    //     core::ColorBackground::new(core::Color::MoselleGreen, core::Color::MoselleGreen),
    //     Some(1i32),
    // );

    let state = vector::FillState::new(
        Into::<core::Paint>::into(style.get_background()),
        core::Matrix2D::default(),
    );

    core::Primitive::new(vertexes, Box::new(state), rectangle)
}

fn export_element_cache(g: &Graphic, element: &crate::Element, export_folder: &str) {
    match element.get_cache() {
        Some(cache) => {
            g.renderer.export_texture(
                cache.get_texture(),
                &format!(
                    "{}/cache/{}.png",
                    export_folder,
                    cache.get_texture().get_id()
                ),
                core::ColorType::Rgba,
            );
        }
        None => {}
    }
}
