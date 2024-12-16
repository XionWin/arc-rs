#[cfg(test)]
pub mod test;

#[allow(dead_code)]
pub(crate) mod gl;

mod attribute_location;
mod call;
mod call_type;
mod def;
mod frag_uniform;
mod frame_data;
mod gl_program;
mod gl_renderer;
mod graphic_rendering;
mod matrix4x3;
mod program_utility;
mod renderer;
mod renderer_utility;
mod shader;
mod texture;
mod texture_rendering;

pub use attribute_location::*;
pub use call::*;
pub use call_type::*;
pub use def::*;
pub use frag_uniform::*;
pub use frame_data::*;
pub use matrix4x3::*;
pub use renderer::*;
pub use texture::*;

pub(crate) use gl_program::*;
pub(crate) use gl_renderer::*;
pub(crate) use graphic_rendering::*;
pub(crate) use shader::*;
pub(crate) use texture_rendering::*;

pub fn load() {
    crate::gl::load();
}
pub fn load_with<T>(loadfn: T)
where
    T: Fn(&str) -> *const std::ffi::c_void,
{
    gl::load_with(loadfn);
}
