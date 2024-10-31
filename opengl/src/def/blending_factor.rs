#[repr(u32)]
pub enum BlendingFactor {
    Zero = gl::ZERO,
    SrcColor = gl::SRC_COLOR,
    OneMinusSrcColor = gl::ONE_MINUS_SRC_COLOR,
    SrcAlpha = gl::SRC_ALPHA,
    OneMinusSrcAlpha = gl::ONE_MINUS_SRC_ALPHA,
    DstAlpha = gl::DST_ALPHA,
    OneMinusDstAlpha = gl::ONE_MINUS_DST_ALPHA,
    DstColor = gl::DST_COLOR,
    OneMinusDstColor = gl::ONE_MINUS_DST_COLOR,
    SrcAlphaSaturate = gl::SRC_ALPHA_SATURATE,
    ConstantColor = gl::CONSTANT_COLOR,
    OneMinusConstantColor = gl::ONE_MINUS_CONSTANT_COLOR,
    ConstantAlpha = gl::CONSTANT_ALPHA,
    OneMinusConstantAlpha = gl::ONE_MINUS_CONSTANT_ALPHA,
    Src1Alpha = gl::SRC1_ALPHA,
    Src1Color = gl::SRC1_COLOR,
    One = gl::ONE
}