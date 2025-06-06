use std::{
    cell::Cell,
    collections::HashMap,
    ffi::{c_int, c_uint},
};

use crate::{program_utility, FragUniform, Shader};

#[derive(Debug)]
pub struct GraphicRenderingProgram {
    viewport: Cell<core::Rectangle<i32>>,
    pub(crate) _program_id: c_uint,
    _vertex_shader: Shader,
    _fragment_shader: Shader,
    _attribute_locations: HashMap<String, c_int>,
}

const VERTEX_SHADER_PATH: &str = "resource/shader/graphic.vert";
const FRAGMENT_SHADER_PATH: &str = "resource/shader/graphic.frag";

impl GraphicRenderingProgram {
    pub fn new() -> Self {
        let _program_id = crate::gl::create_program();
        let _vertex_shader =
            Shader::new(crate::def::ShaderType::VertexShader, VERTEX_SHADER_PATH).load();
        let _fragment_shader =
            Shader::new(crate::def::ShaderType::FragmentShader, FRAGMENT_SHADER_PATH).load();
        program_utility::link_program(_program_id, _vertex_shader.id, _fragment_shader.id);

        let _attribute_locations = program_utility::get_attribute_locations(_program_id);
        Self {
            viewport: Cell::new(core::Rectangle::default()),
            _program_id,
            _vertex_shader,
            _fragment_shader,
            _attribute_locations,
        }
    }

    pub fn use_program(&self) {
        crate::gl::use_program(self._program_id);
    }

    pub fn get_rendering_size(&self) -> core::Size<i32> {
        self.viewport.get().get_size().clone()
    }
    pub fn set_uniform_a_viewport(&self, value: core::Rectangle<i32>) {
        crate::gl::use_program(self._program_id);
        crate::gl::uniform_2f(
            self._attribute_locations["aViewport"],
            value.get_width() as _,
            value.get_height() as _,
        );
        self.viewport.replace(value);
    }
    pub fn set_uniform_a_offset(&self, offset: core::Offset<i32>) {
        crate::gl::use_program(self._program_id);
        crate::gl::uniform_2f(
            self._attribute_locations["aOffset"],
            offset.get_x() as _,
            offset.get_y() as _,
        );
    }
    pub fn set_uniform_frag(&self, value: &FragUniform) {
        crate::gl::use_program(self._program_id);
        crate::gl::uniform_4fv(
            self._attribute_locations["aFrag"],
            &Into::<[f32; 44]>::into(value),
        );
    }
    pub fn use_texture_id(&self, texture_id: c_uint) {
        crate::gl::use_program(self._program_id);
        crate::gl::uniform_1i(self._attribute_locations["aTexture"], 0i32);
        crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, texture_id);
    }
}

impl crate::GLProgram for GraphicRenderingProgram {
    fn get_id(&self) -> c_uint {
        self._program_id
    }
}

impl Drop for GraphicRenderingProgram {
    fn drop(&mut self) {
        crate::gl::delete_program(self._program_id);
        util::print_debug!("graphic_rendering_program {} droped", self._program_id)
    }
}
