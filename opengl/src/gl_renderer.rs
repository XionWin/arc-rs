use std::rc::Rc;
use std::{ffi::c_uint, fmt::Debug};

use graphic::{Texture, TextureFilter};

pub trait GLRenderer: Debug {
    fn get_program(&self) -> &dyn crate::GLProgram;
    fn get_vbo(&self) -> c_uint;
    fn get_attribute_locations(&self) -> &[crate::AttributeLocation];
    fn get_color_type(&self) -> core::ColorType;
    fn init(&self);
    fn begin_render(&self);
    fn render(&self);
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: core::Color);
    fn clear(&self);

    fn create_texture(
        self: Rc<Self>,
        size: core::Size<i32>,
        texture_filter: TextureFilter,
    ) -> Rc<dyn Texture>;
    fn create_texture_from_file(
        self: Rc<Self>,
        path: &str,
        texture_filter: TextureFilter,
    ) -> Rc<dyn Texture>;
    fn drop_texture(&self, texture: &dyn Texture);
    fn add_primitive(&self, primitive: vector::Primitive);
}
