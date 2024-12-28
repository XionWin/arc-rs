use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Renderer {
    _size: core::Size<i32>,
    _cache_renderer: crate::MonochromeRenderer,
    _graphic_renderer: crate::GraphicRenderer,
    _textures: RefCell<Vec<Rc<dyn graphic::Texture>>>,
}

impl Renderer {
    pub fn new<T>(size: core::Size<i32>, loadfn: T) -> Self
    where
        T: Fn(&str) -> *const std::ffi::c_void,
    {
        crate::load_with(loadfn);
        Self {
            _size: size,
            _cache_renderer: crate::MonochromeRenderer::new(),
            _graphic_renderer: crate::GraphicRenderer::new(size),
            _textures: RefCell::new(Vec::new()),
        }
    }
}

impl graphic::Renderer for Renderer {
    fn init(&self) {
        crate::gl::enable_multisample();
        self._graphic_renderer.init();
    }
    fn begin_render(&self) {
        self._graphic_renderer.begin_render();
    }
    fn render(&self) {
        // let _ = self.darw_primitive(vector::Primitive::new(
        //     Box::new([
        //         core::Vertex2::new(200f32, 200f32, 1f32, 0.5f32),
        //         core::Vertex2::new(200f32, 300f32, 1f32, 0.5f32),
        //         core::Vertex2::new(300f32, 300f32, 1f32, 0.5f32),
        //         core::Vertex2::new(300f32, 200f32, 1f32, 0.5f32),
        //     ]),
        //     Box::new(vector::FillState::default()),
        // ));
        self._graphic_renderer.render();
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self._graphic_renderer.viewport(x, y, width, height);
    }
    fn clear_color(&self, color: core::Color) {
        self._graphic_renderer.clear_color(color);
    }
    fn clear(&self) {
        self._graphic_renderer.clear();
    }

    fn create_texture(
        &self,
        size: core::Size<i32>,
        color_type: core::ColorType,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        let texture = Rc::new(crate::Texture::new(size, color_type, texture_filter));
        self._textures.borrow_mut().push(texture.clone());
        texture
    }

    fn create_texture_from_file(
        &self,
        path: &str,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        let texture = Rc::new(crate::Texture::load(path, texture_filter));
        self._textures.borrow_mut().push(texture.clone());
        texture
    }

    fn add_primitive(&self, primitive: vector::Primitive) {
        use crate::GLRenderer;
        self._graphic_renderer.add_primitive(primitive);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        util::print_debug!("gl_renderer droped")
    }
}
