use std::rc::Rc;

#[derive(Debug)]
pub struct Renderer {
    _cache_renderer: Rc<crate::TextureRenderer>,
    _graphic_renderer: Rc<crate::GraphicRenderer>,
}

impl Renderer {
    pub fn new<T>(loadfn: T) -> Self
    where
        T: Fn(&str) -> *const std::ffi::c_void,
    {
        crate::load_with(loadfn);
        Self {
            _cache_renderer: Rc::new(crate::TextureRenderer::new()),
            _graphic_renderer: Rc::new(crate::GraphicRenderer::new()),
        }
    }
}

impl graphic::Renderer for Renderer {
    fn init(&self) {
        use crate::GLRenderer;
        self._graphic_renderer.init();
    }
    fn begin_render(&self) {
        use crate::GLRenderer;
        self._graphic_renderer.begin_render();
    }
    fn render(&self) {
        use crate::GLRenderer;
        self._graphic_renderer.render();
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        use crate::GLRenderer;
        self._graphic_renderer.viewport(x, y, width, height);
    }
    fn clear_color(&self, color: core::Color) {
        use crate::GLRenderer;
        self._graphic_renderer.clear_color(color);
    }
    fn clear(&self) {
        use crate::GLRenderer;
        self._graphic_renderer.clear();
    }

    fn create_texture(
        &self,
        size: core::Size<i32>,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        use crate::GLRenderer;
        self._graphic_renderer
            .clone()
            .create_texture(size, texture_filter)
    }

    fn create_texture_from_file(
        &self,
        path: &str,
        texture_filter: graphic::TextureFilter,
    ) -> Rc<dyn graphic::Texture> {
        use crate::GLRenderer;
        self._graphic_renderer
            .clone()
            .create_texture_from_file(path, texture_filter)
    }

    fn drop_texture(&self, texture: &dyn graphic::Texture) {
        use crate::GLRenderer;
        self._graphic_renderer.drop_texture(texture);
    }

    fn draw_primitive(&self, primitive: vector::Primitive) {
        use crate::GLRenderer;
        self._graphic_renderer.add_primitive(primitive);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        util::print_debug!("gl_renderer droped")
    }
}
