use std::{
    collections::HashMap,
    ffi::{c_int, c_uint},
};

use crate::{program_utility, Shader};

#[derive(Debug)]
pub struct PrimitiveRenderingProgram {
    pub(crate) id: c_uint,
    _vertex_shader: Shader,
    _fragment_shader: Shader,
    _attribute_locations: HashMap<String, c_int>,
}

const VERTEX_SHADER_PATH: &str = "resource/shader/primitive.vert";
const FRAGMENT_SHADER_PATH: &str = "resource/shader/primitive.frag";

impl PrimitiveRenderingProgram {
    pub fn new() -> Self {
        let program_id = crate::gl::create_program();
        let vertex_shader =
            Shader::new(crate::def::ShaderType::VertexShader, VERTEX_SHADER_PATH).load();
        let fragment_shader =
            Shader::new(crate::def::ShaderType::FragmentShader, FRAGMENT_SHADER_PATH).load();
        program_utility::link_program(program_id, vertex_shader.id, fragment_shader.id);

        let attribute_locations = program_utility::get_attribute_locations(program_id);
        Self {
            id: program_id,
            _vertex_shader: vertex_shader,
            _fragment_shader: fragment_shader,
            _attribute_locations: attribute_locations,
        }
    }

    pub fn use_program(&self) {
        crate::gl::use_program(self.id);
    }

    pub fn set_viewport(&self, value: core::Rect<i32>) {
        crate::gl::viewport(
            value.get_x(),
            value.get_y(),
            value.get_width(),
            value.get_height(),
        );
        crate::gl::uniform_2f(
            self._attribute_locations["aViewport"],
            value.get_width() as _,
            value.get_height() as _,
        );
    }
}

impl crate::GLProgram for PrimitiveRenderingProgram {
    fn get_id(&self) -> c_uint {
        self.id
    }
}

impl Drop for PrimitiveRenderingProgram {
    fn drop(&mut self) {
        crate::gl::delete_program(self.id);
        util::print_debug!("texture_rendering_program {} droped", self.id)
    }
}
