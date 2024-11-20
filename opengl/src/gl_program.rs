use std::ffi::c_uint;

use crate::Shader;

#[derive(Debug)]
pub struct Program {
    pub(crate) id: c_uint,
    pub(crate) vertex_shader: Shader,
    pub(crate) fragment_shader: Shader,
}

impl Program {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
        Self {
            id: crate::gl::create_program(),
            vertex_shader: Shader::new(
                crate::def::ShaderType::VertexShader,
                vertex_shader_path,
            )
            .load(),
            fragment_shader: Shader::new(
                crate::def::ShaderType::FragmentShader,
                fragment_shader_path,
            )
            .load(),
        }
    }

    pub fn get_id(&self) -> c_uint {
        self.id
    }
}

impl graphic::Program for Program {
    fn use_program(&self) {
        crate::gl::attach_shader(self.id, self.vertex_shader.id);
        crate::gl::attach_shader(self.id, self.fragment_shader.id);
        crate::gl::link_program(self.id);
        crate::gl::use_program(self.id);
        crate::gl::check_link_status(self.id);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        crate::gl::delete_program(self.id);
        util::print_debug!("program {} droped", self.id)
    }
}
