use std::{collections::HashMap, ffi::c_uint};

use crate::Shader;

#[derive(Debug)]
pub struct Program {
    pub(crate) id: c_uint,
    pub(crate) _vertex_shader: Shader,
    pub(crate) _fragment_shader: Shader,
    _attribute_locations: HashMap<String, c_uint>,
}

impl Program {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
        let program_id = crate::gl::create_program();
        let vertex_shader =
            Shader::new(crate::def::ShaderType::VertexShader, vertex_shader_path).load();
        let fragment_shader =
            Shader::new(crate::def::ShaderType::FragmentShader, fragment_shader_path).load();
        use_program(program_id, vertex_shader.id, fragment_shader.id);
        Self {
            id: program_id,
            _vertex_shader: vertex_shader,
            _fragment_shader: fragment_shader,
            _attribute_locations: get_attribute_locations(program_id),
        }
    }

    pub fn use_program(&self) {
        use_program(self.id, self._vertex_shader.id, self._fragment_shader.id);
    }

    pub fn uniform1(&self) {
        util::print_debug!("{:?}", self._attribute_locations);
    }
}

fn use_program(program_id: c_uint, vertex_shader_id: c_uint, fragment_shader_id: c_uint) {
    crate::gl::attach_shader(program_id, vertex_shader_id);
    crate::gl::attach_shader(program_id, fragment_shader_id);
    crate::gl::link_program(program_id);
    crate::gl::use_program(program_id);
    crate::gl::check_link_status(program_id);
}

fn get_attribute_locations(program_id: c_uint) -> HashMap<String, c_uint> {
    let uniforms_len = crate::gl::get_programiv(
        program_id,
        crate::def::GetProgramParameterName::ActiveUniforms,
    );

    let mut result = HashMap::new();
    for index in 0..uniforms_len {
        let (name, uniform_type, size) = crate::gl::get_active_uniform(program_id, index);
        let name = match name.find('[') {
            Some(index) => name.split_at(index).0,
            None => &name,
        };

        util::print_debug!("{:?}, {:?}, {:?}", name, uniform_type, size);
        result.insert(String::from(name), index);
    }
    result
}

impl Drop for Program {
    fn drop(&mut self) {
        crate::gl::delete_program(self.id);
        util::print_debug!("program {} droped", self.id)
    }
}
