use std::ffi::c_uint;

use crate::GLRenderer;

pub(crate) fn bind_vertex_array(vao: c_uint) {
    crate::gl::bind_vertex_array(vao);
}
pub(crate) fn bind_data<T>(renderer: &dyn GLRenderer, vertices: &[T]) {
    crate::gl::bind_buffer(crate::def::BufferTarget::ArrayBuffer, renderer.get_vbo());
    crate::gl::buffer_data(
        crate::def::BufferTarget::ArrayBuffer,
        vertices,
        crate::def::BufferUsageHint::StaticDraw,
    );

    for attribute_location in renderer.get_attribute_locations() {
        let index = crate::gl::get_attrib_location(
            renderer.get_program().get_id(),
            &attribute_location.name,
        );
        crate::gl::enable_vertex_attrib_array(index);
        crate::gl::vertex_attrib_pointer_f32(
            index,
            attribute_location.len as _,
            false,
            std::mem::size_of::<T>() as _,
            (std::mem::size_of::<f32>() * attribute_location.offset) as _,
        );
    }
}
