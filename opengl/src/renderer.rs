use {
    core::{Color, ColorType, Margin, Size, Texture, TextureFilter},
    vector::Primitive,
};

#[derive(Debug)]
pub struct Renderer {
    _framebuffer_renderer: crate::FramebufferRenderer,
    _graphic_renderer: crate::GraphicRenderer,
}

impl Renderer {
    pub fn new<T>(loadfn: T) -> Self
    where
        T: Fn(&str) -> *const std::ffi::c_void,
    {
        crate::load_with(loadfn);
        Self {
            _framebuffer_renderer: crate::FramebufferRenderer::new(),
            _graphic_renderer: crate::GraphicRenderer::new(),
        }
    }
}

impl graphic::Renderer for Renderer {
    fn init(&self, size: Size<i32>) {
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
        self._graphic_renderer.render();
    }
    fn get_rendering_size(&self) -> Size<i32> {
        self._graphic_renderer.get_rendering_size()
    }
    fn set_rendering_size(&self, size: Size<i32>) {
        self._framebuffer_renderer.set_rendering_size(size);
        self._graphic_renderer.set_rendering_size(size);
    }
    fn clear_color(&self, color: Color) {
        self._graphic_renderer.clear_color(color);
    }
    fn clear(&self) {
        self._graphic_renderer.clear();
    }

    fn create_texture(
        &self,
        size: Size<i32>,
        color_type: ColorType,
        texture_filter: TextureFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn Texture> {
        Box::new(crate::Texture::new(
            size,
            color_type,
            texture_filter,
            is_gen_mipmap,
        ))
    }

    fn create_texture_from_file(
        &self,
        path: &str,
        texture_filter: TextureFilter,
        is_gen_mipmap: bool,
    ) -> Box<dyn Texture> {
        Box::new(crate::Texture::load(path, texture_filter, is_gen_mipmap))
    }

    fn cache_primitive(&self, primitive: Primitive) -> graphic::TextureCache {
        let rect = primitive.get_rectangle();
        let cache = graphic::TextureCache::new(
            rect,
            Margin::default(),
            self.create_texture(
                rect.get_size(),
                ColorType::Rgba,
                TextureFilter::Nearest,
                false,
            ),
        );
        self._framebuffer_renderer
            .add_primitive(primitive, cache.get_texture_rc());
        cache
    }

    fn update_primitive(&self, primitive: Primitive, cache: &graphic::TextureCache) {
        self._framebuffer_renderer
            .add_primitive(primitive, cache.get_texture_rc());
    }

    fn add_primitive(&self, primitive: Primitive) {
        self._graphic_renderer.add_primitive(primitive);
    }

    fn export_texture(&self, texture: &dyn Texture, path: &str, color_type: ColorType) {
        self._framebuffer_renderer
            .export_texture(texture, path, color_type);
    }
    fn check_gl_error(&self) -> String {
        format!("{:?}", crate::gl::get_error())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        util::print_debug!("renderer droped")
    }
}
