pub struct Texture<'a> {
    renderer: &'a dyn crate::Renderer,
    id: i32,
    pub(crate) size: core::Size,
    pub(crate) color_type: core::ColorType,
    pub(crate) texture_filter: core::TextureFilter
}

impl<'a> Texture<'a> {
    pub fn new(renderer: &'a dyn crate::Renderer) -> Self {
        Self {
            renderer,
            id: -1,
            size: core::Size::new(10, 10),
            color_type: core::ColorType::Rgba,
            texture_filter: core::TextureFilter::Linear
        }
    }
}

impl<'a> core::Texture for Texture<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_size(&self) -> core::Size {
        self.size
    }

    fn get_color_type(&self) -> core::ColorType {
        self.color_type
    }

    fn get_texture_filter(&self) -> core::TextureFilter {
        self.texture_filter
    }

    fn export(&self, path: &str) {
        todo!()
    }
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) {
        util::print_debug!("texture {} droped", self.id);
        self.renderer.drop_texture(self)
    }
}