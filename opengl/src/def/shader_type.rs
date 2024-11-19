#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ShaderType
{
    FragmentShader = 35632,
    VertexShader = 35633,
    GeometryShader = 36313,
    TessEvaluationShader = 36487,
    TessControlShader = 36488,
    ComputeShader = 37305
}

#[allow(non_upper_case_globals)]
impl ShaderType {
    pub const FragmentShaderArb: ShaderType = ShaderType::FragmentShader;
    pub const VertexShaderArb: ShaderType = ShaderType::VertexShader;
    
}