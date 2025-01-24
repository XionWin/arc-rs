use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Renderer {
    _framebuffer_renderer: crate::FramebufferRenderer,
    _graphic_renderer: crate::GraphicRenderer,
    _textures: RefCell<Vec<Rc<dyn graphic::Texture>>>,
}

impl Renderer {
    pub fn new<T>(loadfn: T) -> Self
    where
        T: Fn(&str) -> *const std::ffi::c_void,
    {
        crate::load_with(loadfn);
        Self {
            _framebuffer_renderer: crate::FramebufferRenderer::new(core::ColorType::Rgba),
            _graphic_renderer: crate::GraphicRenderer::new(),
            _textures: RefCell::new(Vec::new()),
        }
    }
}

impl graphic::Renderer for Renderer {
    fn init(&self, size: core::Size<i32>) {
        crate::gl::enable_multisample();
        self._framebuffer_renderer.init(size);
        self._graphic_renderer.init(size);
    }
    fn begin_render(&self) {
        use crate::GLRenderer;
        self._framebuffer_renderer.begin_render();
        self._graphic_renderer.begin_render();
    }
    fn render(&self) {
        use crate::GLRenderer;
        self._framebuffer_renderer.render();
        for primitive in self._framebuffer_renderer.get_rendered_primivitive() {
            self._graphic_renderer.add_primitive(primitive);
        }
        self._graphic_renderer.render();
    }
    fn get_rendering_size(&self) -> core::Size<i32> {
        self._graphic_renderer.get_rendering_size()
    }
    fn set_rendering_size(&self, size: core::Size<i32>) {
        self._framebuffer_renderer.set_rendering_size(size);
        self._graphic_renderer.set_rendering_size(size);
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

    fn cache_primitive(&self, primitive: vector::Primitive) -> graphic::TextureCache {
        let rect = primitive.get_rect();
        let cache = graphic::TextureCache::new(
            rect,
            core::Margin::default(),
            self.create_texture(
                rect.get_size(),
                core::ColorType::Rgba,
                graphic::TextureFilter::Nearest,
            ),
        );
        self._framebuffer_renderer
            .add_primitive(primitive, cache.get_texture_rc());
        cache
    }

    fn add_primitive(&self, primitive: vector::Primitive) {
        // let texture = self.create_texture_from_file(
        //     "resource/image/icon/icon96.png",
        //     graphic::TextureFilter::Nearest,
        // );
        // self._framebuffer_renderer
        //     .draw_primitive(primitive, texture);
        self._graphic_renderer.add_primitive(primitive);
    }

    fn export_texture(
        &self,
        texture: &dyn graphic::Texture,
        path: &str,
        color_type: core::ColorType,
    ) {
        self._framebuffer_renderer
            .export_texture(texture, path, color_type);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        util::print_debug!("renderer droped")
    }
}
