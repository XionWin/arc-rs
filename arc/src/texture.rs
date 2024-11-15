pub struct Texture {
    graphic: &'static dyn crate::Graphic,
    id: i32,
    size: core::Size,
    color_type: core::ColorType,
    texture_filter: core::TextureFilter
}

impl core::Texture for Texture {
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

impl Drop for Texture {
    fn drop(&mut self) {
        util::print_debug!("texture {} droped", self.id);
        self.graphic.drop_texture(self)
    }
}