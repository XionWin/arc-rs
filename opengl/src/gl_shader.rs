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

    pub(crate) fn load(self) -> Self {
        let source = String::from(&self.source);
        let source_cstring = std::ffi::CString::new(source).unwrap();
        let sources = vec![source_cstring.as_ptr()];
        unsafe {
            gl::ShaderSource(self.id, 1, sources.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
        }
        check_compile(self)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
    
fn check_compile(shader: super::Shader) -> super::Shader {
    let mut is_compiled = 0;
    unsafe {
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut is_compiled);
    }
    if is_compiled == 0 {
        panic!("GLES shader compile faild");
    }
    return shader;
}