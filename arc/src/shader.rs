pub trait SourceShader {
    fn attach_vertex_shader(&mut self, vertex_shader: &str);
    fn attach_fragment_shader(&mut self, fragment_shader: &str);
    fn create_program(&mut self);
    fn compile(&self) -> Result<Box<dyn CompiledShader>, String>;
}

pub trait CompiledShader {
    fn link(&self) -> Box<dyn Shader>;
}

pub trait Shader {
    fn use_program(&self);
}
