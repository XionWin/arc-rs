use std::sync::Arc;

pub struct Texture<'a> {
    renderer: Arc<&'a dyn arc::Renderer>,
    id: i32,
    pub(crate) size: core::Size<i32>,
    pub(crate) color_type: core::ColorType,
    pub(crate) texture_filter: core::TextureFilter
}

impl<'a> Texture<'a> {
    pub fn new(renderer: Arc<&'a dyn arc::Renderer>, size: core::Size<i32>, color_type: core::ColorType, texture_filter: core::TextureFilter) -> Self {
        Self {
            renderer,
            id: 1,
            size,
            color_type,
            texture_filter
        }
    }
}

impl<'a> core::Texture for Texture<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_size(&self) -> core::Size<i32> {
        self.size
    }

    fn get_color_type(&self) -> core::ColorType {
        self.color_type
    }

    fn get_texture_filter(&self) -> core::TextureFilter {
        self.texture_filter
    }

    fn export(&self, path: &str) {
        println!("path: {}", path);
        todo!()
    }
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) {
        util::print_debug!("texture {} droped", self.id);
        self.renderer.drop_texture(self);
    }
}