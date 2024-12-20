use std::rc::Rc;

#[derive(Debug)]
pub struct Renderer {
    _cache_renderer: crate::TextureRenderer,
    _graphic_renderer: crate::GraphicRenderer,
}

impl Renderer {
    pub fn new<T>(loadfn: T) -> Self
    where
        T: Fn(&str) -> *const std::ffi::c_void,
    {
        crate::load_with(loadfn);
        Self {
            _cache_renderer: crate::TextureRenderer::new(),
            _graphic_renderer: crate::GraphicRenderer::new(),
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
        let _ = self.darw_primitive(vector::Primitive::new(
            Box::new([
                core::Vertex2::new(200f32, 200f32, 1f32, 0.5f32),
                core::Vertex2::new(200f32, 300f32, 1f32, 0.5f32),
                core::Vertex2::new(300f32, 300f32, 1f32, 0.5f32),
                core::Vertex2::new(300f32, 200f32, 1f32, 0.5f32),
            ]),
            Box::new(vector::FillState::default()),
        ));
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
        Rc::new(crate::Texture::new(size, color_type, texture_filter))
    }

    fn create_texture_from_file(
        &self,
        path: &str,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        Rc::new(crate::Texture::load(path, texture_filter))
    }

    fn darw_primitive(&self, primitive: vector::Primitive) -> graphic::TextureCache {
        self._cache_renderer.draw_primitive_texture(&primitive)
    }

    fn add_primitive(&self, primitive: vector::Primitive) {
        self._graphic_renderer.add_primitive(primitive);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        util::print_debug!("gl_renderer droped")
    }
}
