use std::{ffi::c_uint, fmt::Debug};

pub trait GLRenderer: Debug {
    fn get_program(&self) -> &dyn crate::GLProgram;
    fn get_vbo(&self) -> c_uint;
    fn get_attribute_locations(&self) -> &[crate::AttributeLocation];
    #[allow(dead_code)]
    fn get_color_type(&self) -> core::ColorType;
    fn begin_render(&self);
    fn render(&self);
}
