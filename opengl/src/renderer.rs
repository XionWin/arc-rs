use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Renderer {
    _cache_renderer: crate::PrimitiveRenderer,
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
            _cache_renderer: crate::PrimitiveRenderer::new(core::ColorType::Rgba),
            _graphic_renderer: crate::GraphicRenderer::new(),
            _textures: RefCell::new(Vec::new()),
        }
    }
}

impl graphic::Renderer for Renderer {
    fn init(&self, size: core::Size<i32>) {
        crate::gl::enable_multisample();
        self._graphic_renderer.init(size);
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
    fn get_rendering_size(&self) -> core::Size<i32> {
        self._graphic_renderer.get_rendering_size()
    }
    fn set_rendering_size(&self, size: core::Size<i32>) {
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

    fn draw_primitive_cache(&self, primitive: vector::Primitive, cache: &graphic::TextureCache) {
        use crate::GLPrimitiveRenderer;
        self._cache_renderer
            .draw_primitive_on_texture(primitive, cache.get_texture());
    }

    fn add_primitive(&self, primitive: vector::Primitive) {
        use crate::GLGraphicRenderer;
        self._graphic_renderer.add_primitive(primitive);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        util::print_debug!("renderer droped")
    }
}
