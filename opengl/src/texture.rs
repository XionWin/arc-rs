use std::rc::Rc;

pub struct Texture {
    renderer: Rc<dyn arc::Renderer>,
    id: u32,
    size: core::Size<i32>,
    color_type: core::ColorType,
    texture_filter: arc::TextureFilter
}

impl Texture {
    pub fn new(renderer: Rc<dyn arc::Renderer>, size: core::Size<i32>, color_type: core::ColorType, texture_filter: arc::TextureFilter) -> Self {
        Self {
            renderer,
            id: crate::gl::gen_texture(),
            size,
            color_type,
            texture_filter
        }
    }
}

impl arc::Texture for Texture {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_size(&self) -> core::Size<i32> {
        self.size
    }

    fn get_color_type(&self) -> core::ColorType {
        self.color_type
    }

    fn get_texture_filter(&self) -> arc::TextureFilter {
        self.texture_filter
    }

    fn export(&self, path: &str) {
        println!("path: {}", path);
        todo!()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.renderer.drop_texture(self);
        util::print_debug!("texture {} droped", self.id);
    }
}