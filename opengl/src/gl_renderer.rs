use std::{ffi::c_uint, fmt::Debug};

pub trait GLRenderer: Debug {
    fn get_program(&self) -> &dyn crate::GLProgram;
    fn get_vbo(&self) -> c_uint;
    fn get_attribute_locations(&self) -> &[crate::AttributeLocation];
    #[allow(dead_code)]
    fn get_color_type(&self) -> core::ColorType;
}

pub trait GLGraphicRenderer: GLRenderer {
    fn add_primitive(&self, primitive: vector::Primitive);
}

pub trait GLPrimitiveRenderer: GLRenderer {
    fn draw_primitive_on_texture(
        &self,
        primitive: vector::Primitive,
        texture: &dyn graphic::Texture,
    );
}
