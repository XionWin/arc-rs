#[repr(u32)]
#[derive(Debug)]
pub enum ActiveUniformType {
    //
    // Summary:
    //     Original was GL_Int = 0X1404
    Int = 5124,
    //
    // Summary:
    //     Original was GL_Float = 0X1406
    Float = 5126,
    //
    // Summary:
    //     Original was GL_FloatVec2 = 0X8b50
    FloatVec2 = 35664,
    //
    // Summary:
    //     Original was GL_FloatVec3 = 0X8b51
    FloatVec3 = 35665,
    //
    // Summary:
    //     Original was GL_FloatVec4 = 0X8b52
    FloatVec4 = 35666,
    //
    // Summary:
    //     Original was GL_IntVec2 = 0X8b53
    IntVec2 = 35667,
    //
    // Summary:
    //     Original was GL_IntVec3 = 0X8b54
    IntVec3 = 35668,
    //
    // Summary:
    //     Original was GL_IntVec4 = 0X8b55
    IntVec4 = 35669,
    //
    // Summary:
    //     Original was GL_Bool = 0X8b56
    Bool = 35670,
    //
    // Summary:
    //     Original was GL_BoolVec2 = 0X8b57
    BoolVec2 = 35671,
    //
    // Summary:
    //     Original was GL_BoolVec3 = 0X8b58
    BoolVec3 = 35672,
    //
    // Summary:
    //     Original was GL_BoolVec4 = 0X8b59
    BoolVec4 = 35673,
    //
    // Summary:
    //     Original was GL_FloatMat2 = 0X8b5a
    FloatMat2 = 35674,
    //
    // Summary:
    //     Original was GL_FloatMat3 = 0X8b5b
    FloatMat3 = 35675,
    //
    // Summary:
    //     Original was GL_FloatMat4 = 0X8b5c
    FloatMat4 = 35676,
    //
    // Summary:
    //     Original was GL_Sampler2D = 0X8b5e
    Sampler2D = 35678,
    //
    // Summary:
    //     Original was GL_SamplerCube = 0X8b60
    SamplerCube = 35680,
}

impl From<u32> for ActiveUniformType {
    fn from(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
