mod def;
mod frag_uniform;
#[allow(dead_code)]
pub(crate) mod gl;
mod gl_program;
mod gl_renderer;
mod gl_shader;
mod matrix4x3;
mod texture;

pub use def::*;
pub use frag_uniform::*;
pub(crate) use gl_program::*;
pub use gl_renderer::*;
pub(crate) use gl_shader::*;
pub use matrix4x3::*;
pub use texture::*;

pub fn load() {
    crate::gl::load();
}
pub fn load_with<T>(loadfn: T)
where
    T: Fn(&str) -> *const std::ffi::c_void,
{
    gl::load_with(loadfn);
}
