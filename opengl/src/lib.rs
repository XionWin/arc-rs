mod def;
#[allow(dead_code)]
pub(crate) mod gl;
mod gl_renderer;
mod texture;
mod gl_shader;
mod gl_program;

pub use def::*;
pub use gl_renderer::*;
pub use texture::*;
pub(crate) use gl_shader::*;
pub use gl_program::*;


pub fn load() {
    crate::gl::load();
}
pub fn load_with<T>(loadfn: T)
where
    T: Fn(&str) -> *const std::ffi::c_void,
{
    gl::load_with(loadfn);
}
