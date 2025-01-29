#[cfg(test)]
pub mod test;

#[allow(dead_code)]
pub(crate) mod gl;

mod attribute_location;
mod call_type;
mod def;
mod frag_uniform;
mod framebuffer_rendering;
mod gl_program;
mod gl_renderer;
mod graphic_rendering;
mod matrix4x3;
mod program_utility;
mod renderer;
mod renderer_utility;
mod shader;
mod texture;

use std::{ffi::c_int, sync::RwLock};

pub use attribute_location::*;
pub use call_type::*;
pub use def::*;
pub use frag_uniform::*;
pub use matrix4x3::*;
pub use renderer::*;
pub use texture::*;

pub(crate) use framebuffer_rendering::*;
pub(crate) use gl_program::*;
pub(crate) use gl_renderer::*;
pub(crate) use graphic_rendering::*;
pub(crate) use shader::*;

pub fn load() {
    crate::gl::load();
}
pub fn load_with<T>(loadfn: T)
where
    T: Fn(&str) -> *const std::ffi::c_void,
{
    gl::load_with(loadfn);
}

static IS_ENABLE_MILTISAMPLE_FRAMEBUFFER: RwLock<bool> = RwLock::new(false);

pub fn get_is_enable_multisample() -> bool {
    let is_enable_miltisample_framebuffer =
        crate::IS_ENABLE_MILTISAMPLE_FRAMEBUFFER.read().unwrap();
    *is_enable_miltisample_framebuffer
}

pub fn set_is_enable_multisample(value: bool) {
    let mut is_enable_miltisample_framebuffer =
        crate::IS_ENABLE_MILTISAMPLE_FRAMEBUFFER.write().unwrap();
    *is_enable_miltisample_framebuffer = value
}

pub fn get_max_samples() -> c_int {
    gl::get_max_samples()
}
