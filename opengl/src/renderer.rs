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
        self._graphic_renderer.init();
    }
    fn begin_render(&self) {
        self._graphic_renderer.begin_render();
    }
    fn render(&self) {
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
        self._graphic_renderer
            .create_texture(size, color_type, texture_filter)
    }

    fn create_texture_from_file(
        &self,
        path: &str,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        self._graphic_renderer
            .create_texture_from_file(path, texture_filter)
    }

    fn draw_primitive(&self, primitive: vector::Primitive) {
        self._graphic_renderer.add_primitive(primitive);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        util::print_debug!("gl_renderer droped")
    }
}
