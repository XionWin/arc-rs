#[cfg(test)]
pub mod test;

#[allow(dead_code)]
pub(crate) mod gl;

mod call;
mod call_type;
mod def;
mod frag_uniform;
mod frame_data;
mod gl_program;
mod matrix4x3;
mod texture;
mod texture_image;

pub use call::*;
pub use call_type::*;
pub use def::*;
pub use frag_uniform::*;
pub use frame_data::*;
pub use gl_renderer::*;
pub use matrix4x3::*;
pub use texture::*;
pub use texture_image::*;

mod gl_renderer;
mod gl_shader;
pub(crate) use gl_program::*;
pub(crate) use gl_shader::*;

pub fn load() {
    crate::gl::load();
}
pub fn load_with<T>(loadfn: T)
where
    T: Fn(&str) -> *const std::ffi::c_void,
{
    gl::load_with(loadfn);
}
