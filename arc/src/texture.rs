pub struct Texture<'a> {
    graphic: &'a dyn crate::Graphic,
    id: i32,
    size: core::Size,
    color_type: core::ColorType,
    texture_filter: core::TextureFilter
}

impl<'a> Texture<'a> {
    pub fn new(graphic: &'a dyn crate::Graphic) -> Self {
        Self {
            graphic,
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
        self.graphic.drop_texture(self)
    }
}