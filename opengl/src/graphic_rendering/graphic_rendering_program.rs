use std::{
    collections::HashMap,
    ffi::{c_int, c_uint},
};

use crate::{FragUniform, Shader};

#[derive(Debug)]
pub struct GraphicRenderingProgram {
    pub(crate) id: c_uint,
    _vertex_shader: Shader,
    _fragment_shader: Shader,
    _attribute_locations: HashMap<String, c_int>,
}

const VERTEX_SHADER_PATH: &str = "resource/shader/graphic.vert";
const FRAGMENT_SHADER_PATH: &str = "resource/shader/graphic.frag";

impl GraphicRenderingProgram {
    pub fn new() -> Self {
        let program_id = crate::gl::create_program();
        let vertex_shader =
            Shader::new(crate::def::ShaderType::VertexShader, VERTEX_SHADER_PATH).load();
        let fragment_shader =
            Shader::new(crate::def::ShaderType::FragmentShader, FRAGMENT_SHADER_PATH).load();
        link_program(program_id, vertex_shader.id, fragment_shader.id);

        let attribute_locations = get_attribute_locations(program_id);
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
            value.location.x,
            value.location.y,
            value.size.width,
            value.size.height,
        );
        crate::gl::uniform_2f(
            self._attribute_locations["aViewport"],
            value.size.width as _,
            value.size.height as _,
        );
        crate::gl::uniform_2f(self._attribute_locations["aOffset"], 0f32, 0f32);
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
    pub fn set_texture_id(&self, texture_id: c_uint) {
        crate::gl::uniform_1i(self._attribute_locations["aTexture"], 0i32);
        crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, texture_id);
    }
}

fn link_program(program_id: c_uint, vertex_shader_id: c_uint, fragment_shader_id: c_uint) {
    crate::gl::attach_shader(program_id, vertex_shader_id);
    crate::gl::attach_shader(program_id, fragment_shader_id);
    crate::gl::link_program(program_id);
    crate::gl::check_link_status(program_id);
}

fn get_attribute_locations(program_id: c_uint) -> HashMap<String, c_int> {
    let uniforms_len = crate::gl::get_programiv(
        program_id,
        crate::def::GetProgramParameterName::ActiveUniforms,
    );

    let mut result = HashMap::new();
    for index in 0..uniforms_len {
        let (name, uniform_type, size) = crate::gl::get_active_uniform(program_id, index as _);
        let name = match name.find('[') {
            Some(index) => name.split_at(index).0,
            None => &name,
        };

        util::print_debug!("{:?}, {:?}, {:?}", name, uniform_type, size);
        result.insert(String::from(name), index);
    }
    result
}

impl crate::GLProgram for GraphicRenderingProgram {
    fn get_id(&self) -> c_uint {
        self.id
    }
}

impl Drop for GraphicRenderingProgram {
    fn drop(&mut self) {
        crate::gl::delete_program(self.id);
        util::print_debug!("primitive_program {} droped", self.id)
    }
}
