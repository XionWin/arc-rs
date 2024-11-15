use opengl::gl;

pub struct Graphic {
}

impl Graphic {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl arc::Graphic for Graphic {

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        println!("call viewport function")
    }

    fn clear_color(&self, color: arc::Color) {
        let rgba: arc::Rgba = color.into();
        let (r, g, b, a) = rgba.into();
        gl::clear_color(r, g, b, a);
    }

    fn clear(&self, ) {
        gl::clear(opengl::ClearBufferMask::COLOR_BUFFER_BIT | opengl::ClearBufferMask::DEPTH_BUFFER_BIT);
    }

    fn load_image(&self, path: &str, color_type: core::ColorType, color_filter: core::TextureFilter) {
        todo!()
    }

    fn load_image_data(&self, image_data: core::ImageData, color_filter: core::TextureFilter) {
        todo!()
    }

    fn drop_texture(&self, texture: &dyn core::Texture) {
        todo!()
    }
}