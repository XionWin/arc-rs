use std::{
    cell::Cell,
    collections::HashMap,
    ffi::{c_int, c_uint},
};

use crate::{program_utility, FragUniform, Shader};

#[derive(Debug)]
pub struct FramebufferRenderingProgram {
    viewport: Cell<core::Rect<i32>>,
    pub(crate) id: c_uint,
    _vertex_shader: Shader,
    _fragment_shader: Shader,
    _attribute_locations: HashMap<String, c_int>,
}

const VERTEX_SHADER_PATH: &str = "resource/shader/graphic.vert";
const FRAGMENT_SHADER_PATH: &str = "resource/shader/graphic.frag";

impl FramebufferRenderingProgram {
    pub fn new() -> Self {
        let program_id = crate::gl::create_program();
        let vertex_shader =
            Shader::new(crate::def::ShaderType::VertexShader, VERTEX_SHADER_PATH).load();
        let fragment_shader =
            Shader::new(crate::def::ShaderType::FragmentShader, FRAGMENT_SHADER_PATH).load();
        program_utility::link_program(program_id, vertex_shader.id, fragment_shader.id);

        let attribute_locations = program_utility::get_attribute_locations(program_id);
        Self {
            viewport: Cell::new(core::Rect::default()),
            id: program_id,
            _vertex_shader: vertex_shader,
            _fragment_shader: fragment_shader,
            _attribute_locations: attribute_locations,
        }
    }

    pub fn use_program(&self) {
        crate::gl::use_program(self.id);
    }

    pub fn get_rendering_size(&self) -> core::Size<i32> {
        self.viewport.get().get_size().clone()
    }
    pub fn set_uniform_a_viewport(&self, value: core::Rect<i32>) {
        crate::gl::uniform_2f(
            self._attribute_locations["aViewport"],
            value.get_width() as _,
            value.get_height() as _,
        );
        crate::gl::uniform_2f(self._attribute_locations["aOffset"], 0f32, 0f32);
        self.viewport.replace(value);
    }

    pub fn set_uniform_point_size(&self, value: std::ffi::c_int) {
        crate::gl::uniform_1i(self._attribute_locations["aPointSize"], value);
    }
    pub fn set_uniform_frag(&self, value: &FragUniform) {
        crate::gl::uniform_4fv(
            self._attribute_locations["aFrag"],
            &Into::<[f32; 44]>::into(value),
        );
    }
    pub fn use_texture_id(&self, texture_id: c_uint) {
        crate::gl::uniform_1i(self._attribute_locations["aTexture"], 0i32);
        crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, texture_id);
    }
}

impl crate::GLProgram for FramebufferRenderingProgram {
    fn get_id(&self) -> c_uint {
        self.id
    }
}

impl Drop for FramebufferRenderingProgram {
    fn drop(&mut self) {
        crate::gl::delete_program(self.id);
        util::print_debug!("framebuffer_rendering_program {} droped", self.id)
    }
}
