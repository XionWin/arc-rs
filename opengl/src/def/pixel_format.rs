#[repr(u32)]
pub enum PixelFormat {
    UnsignedShort = gl::UNSIGNED_SHORT,
    UnsignedInt = gl::UNSIGNED_INT,
    ColorIndex = 6400,
    StencilIndex = gl::STENCIL_INDEX,
    DepthComponent = gl::DEPTH_COMPONENT,
    Red = gl::RED,
    Green = gl::GREEN,
    Blue = gl::BLUE,
    Alpha = gl::ALPHA,
    Rgb = gl::RGB,
    Rgba = gl::RGBA,
    Luminance = 6409,
    LuminanceAlpha = 6410,
    AbgrExt = 32768,
    CmykExt = 32780,
    CmykaExt = 32781,
    Ycrcb422Sgix = 33211,
    Ycrcb444Sgix = 33212
}