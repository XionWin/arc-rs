use std::{
    collections::HashMap,
    ffi::{c_int, c_uint},
};

pub(crate) fn link_program(
    program_id: c_uint,
    vertex_shader_id: c_uint,
    fragment_shader_id: c_uint,
) {
    crate::gl::attach_shader(program_id, vertex_shader_id);
    crate::gl::attach_shader(program_id, fragment_shader_id);
    crate::gl::link_program(program_id);
    crate::gl::check_link_status(program_id);
}

pub(crate) fn get_attribute_locations(program_id: c_uint) -> HashMap<String, c_int> {
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
