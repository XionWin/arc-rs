#[repr(u32)]
pub enum PixelFormat {
    //
    // Summary:
    //     [requires: ANGLE_depth_texture, or OES_depth_texture] Original was GL_UNSIGNED_SHORT
    //     = 0x1403
    UnsignedShort = 5123,
    //
    // Summary:
    //     [requires: ANGLE_depth_texture, or OES_depth_texture, OES_element_index_uint]
    //     Original was GL_UNSIGNED_INT = 0x1405
    UnsignedInt = 5125,
    //
    // Summary:
    //     Original was GL_COLOR_INDEX = 0x1900
    ColorIndex = 6400,
    //
    // Summary:
    //     Original was GL_STENCIL_INDEX = 0x1901
    StencilIndex = 6401,
    //
    // Summary:
    //     [requires: ANGLE_depth_texture, or OES_depth_texture] Original was GL_DEPTH_COMPONENT
    //     = 0x1902
    DepthComponent = 6402,
    //
    // Summary:
    //     Original was GL_RED = 0x1903
    Red = 6403,
    //
    // Summary:
    //     Original was GL_GREEN = 0x1904
    Green = 6404,
    //
    // Summary:
    //     Original was GL_BLUE = 0x1905
    Blue = 6405,
    //
    // Summary:
    //     Original was GL_Alpha = 0X1906
    Alpha = 6406,
    //
    // Summary:
    //     Original was GL_Rgb = 0X1907
    Rgb = 6407,
    //
    // Summary:
    //     Original was GL_Rgba = 0X1908
    Rgba = 6408,
    //
    // Summary:
    //     Original was GL_Luminance = 0X1909
    Luminance = 6409,
    //
    // Summary:
    //     Original was GL_LUMINANCE_ALPHA = 0x190A
    LuminanceAlpha = 6410,
    //
    // Summary:
    //     Original was GL_ABGR_EXT = 0x8000
    AbgrExt = 32768,
    //
    // Summary:
    //     Original was GL_CMYK_EXT = 0x800C
    CmykExt = 32780,
    //
    // Summary:
    //     Original was GL_CMYKA_EXT = 0x800D
    CmykaExt = 32781,
    //
    // Summary:
    //     Original was GL_YCRCB_422_SGIX = 0x81BB
    Ycrcb422Sgix = 33211,
    //
    // Summary:
    //     Original was GL_YCRCB_444_SGIX = 0x81BC
    Ycrcb444Sgix = 33212,
}

#[allow(non_upper_case_globals)]
impl PixelFormat {
    pub const RedExt: PixelFormat = PixelFormat::Red;
}
