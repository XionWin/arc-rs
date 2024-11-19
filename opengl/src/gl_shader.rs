use std::ffi::c_uint;

#[derive(Debug)]
pub struct Shader
{
    pub id: c_uint,
    pub source: String,
    pub shader_type: crate::def::ShaderType,
}

impl Shader {
    pub fn new(shader_type: crate::def::ShaderType, path: &str) -> Self {
        Self {
            id: unsafe {
                gl::CreateShader(shader_type as _)
            },
            source: std::fs::read_to_string(path)
            .expect("unread the file"),
            shader_type,
        }
    }

    pub fn load(self) -> Self {
        let source = String::from(&self.source);
        let source_cstring = std::ffi::CString::new(source).unwrap();
        let sources = vec![source_cstring.as_ptr()];
        unsafe {
            gl::ShaderSource(self.id, 1, sources.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
        }
        check_compile(self)
    }

    #[allow(dead_code)]
    pub fn get_shader_type(&self) -> crate::def::ShaderType {
        self.shader_type
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
            util::print_debug!("shader {} droped", self.id)
        }
    }
}
    
fn check_compile(shader: crate::Shader) -> crate::Shader {
    let mut is_compiled = 0;
    unsafe {
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut is_compiled);
    }
    if is_compiled == 0 {
        util::print_panic!("shader compile faild");
    }
    return shader;
}