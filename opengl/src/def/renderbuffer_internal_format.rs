#[repr(u32)]
pub enum RenderbufferInternalFormat {
    //
    // Summary:
    //     Original was GL_RGB8 = 0x8051
    Rgb8 = 32849,
    //
    // Summary:
    //     Original was GL_Rgba4 = 0X8056
    Rgba4 = 32854,
    //
    // Summary:
    //     Original was GL_RGB5_A1 = 0x8057
    Rgb5A1 = 32855,
    //
    // Summary:
    //     Original was GL_RGBA8 = 0x8058
    Rgba8 = 32856,
    //
    // Summary:
    //     Original was GL_RGB10_A2 = 0x8059
    Rgb10A2 = 32857,
    //
    // Summary:
    //     [requires: ANGLE_depth_texture] Original was GL_DEPTH_COMPONENT16 = 0x81A5
    DepthComponent16 = 33189,
    //
    // Summary:
    //     Original was GL_DEPTH_COMPONENT24 = 0x81A6
    DepthComponent24 = 33190,
    //
    // Summary:
    //     Original was GL_R8 = 0x8229
    R8 = 33321,
    //
    // Summary:
    //     Original was GL_RG8 = 0x822B
    Rg8 = 33323,
    //
    // Summary:
    //     Original was GL_R16F = 0x822D
    R16f = 33325,
    //
    // Summary:
    //     Original was GL_R32F = 0x822E
    R32f = 33326,
    //
    // Summary:
    //     Original was GL_RG16F = 0x822F
    Rg16f = 33327,
    //
    // Summary:
    //     Original was GL_RG32F = 0x8230
    Rg32f = 33328,
    //
    // Summary:
    //     Original was GL_R8I = 0x8231
    R8i = 33329,
    //
    // Summary:
    //     Original was GL_R8UI = 0x8232
    R8ui = 33330,
    //
    // Summary:
    //     Original was GL_R16I = 0x8233
    R16i = 33331,
    //
    // Summary:
    //     Original was GL_R16UI = 0x8234
    R16ui = 33332,
    //
    // Summary:
    //     Original was GL_R32I = 0x8235
    R32i = 33333,
    //
    // Summary:
    //     Original was GL_R32UI = 0x8236
    R32ui = 33334,
    //
    // Summary:
    //     Original was GL_RG8I = 0x8237
    Rg8i = 33335,
    //
    // Summary:
    //     Original was GL_RG8UI = 0x8238
    Rg8ui = 33336,
    //
    // Summary:
    //     Original was GL_RG16I = 0x8239
    Rg16i = 33337,
    //
    // Summary:
    //     Original was GL_RG16UI = 0x823A
    Rg16ui = 33338,
    //
    // Summary:
    //     Original was GL_RG32I = 0x823B
    Rg32i = 33339,
    //
    // Summary:
    //     Original was GL_RG32UI = 0x823C
    Rg32ui = 33340,
    //
    // Summary:
    //     Original was GL_RGBA32F = 0x8814
    Rgba32f = 34836,
    //
    // Summary:
    //     Original was GL_RGB32F = 0x8815
    Rgb32f = 34837,
    //
    // Summary:
    //     Original was GL_RGBA16F = 0x881A
    Rgba16f = 34842,
    //
    // Summary:
    //     Original was GL_RGB16F = 0x881B
    Rgb16f = 34843,
    //
    // Summary:
    //     Original was GL_DEPTH24_STENCIL8 = 0x88F0
    Depth24Stencil8 = 35056,
    //
    // Summary:
    //     Original was GL_R11F_G11F_B10F = 0x8C3A
    R11fG11fB10f = 35898,
    //
    // Summary:
    //     Original was GL_RGB9_E5 = 0x8C3D
    Rgb9E5 = 35901,
    //
    // Summary:
    //     Original was GL_SRGB8 = 0x8C41
    Srgb8 = 35905,
    //
    // Summary:
    //     Original was GL_SRGB8_ALPHA8 = 0x8C43
    Srgb8Alpha8 = 35907,
    //
    // Summary:
    //     Original was GL_DEPTH_COMPONENT32F = 0x8CAC
    DepthComponent32f = 36012,
    //
    // Summary:
    //     Original was GL_DEPTH32F_STENCIL8 = 0x8CAD
    Depth32fStencil8 = 36013,
    //
    // Summary:
    //     Original was GL_StencilIndex8 = 0X8d48
    StencilIndex8 = 36168,
    //
    // Summary:
    //     Original was GL_Rgb565 = 0X8d62
    Rgb565 = 36194,
    //
    // Summary:
    //     Original was GL_RGBA32UI = 0x8D70
    Rgba32ui = 36208,
    //
    // Summary:
    //     Original was GL_RGB32UI = 0x8D71
    Rgb32ui = 36209,
    //
    // Summary:
    //     Original was GL_RGBA16UI = 0x8D76
    Rgba16ui = 36214,
    //
    // Summary:
    //     Original was GL_RGB16UI = 0x8D77
    Rgb16ui = 36215,
    //
    // Summary:
    //     Original was GL_RGBA8UI = 0x8D7C
    Rgba8ui = 36220,
    //
    // Summary:
    //     Original was GL_RGB8UI = 0x8D7D
    Rgb8ui = 36221,
    //
    // Summary:
    //     Original was GL_RGBA32I = 0x8D82
    Rgba32i = 36226,
    //
    // Summary:
    //     Original was GL_RGB32I = 0x8D83
    Rgb32i = 36227,
    //
    // Summary:
    //     Original was GL_RGBA16I = 0x8D88
    Rgba16i = 36232,
    //
    // Summary:
    //     Original was GL_RGB16I = 0x8D89
    Rgb16i = 36233,
    //
    // Summary:
    //     Original was GL_RGBA8I = 0x8D8E
    Rgba8i = 36238,
    //
    // Summary:
    //     Original was GL_RGB8I = 0x8D8F
    Rgb8i = 36239,
    //
    // Summary:
    //     [requires: EXT_render_snorm] Original was GL_R8_SNORM = 0x8F94
    R8Snorm = 36756,
    //
    // Summary:
    //     [requires: EXT_render_snorm] Original was GL_RG8_SNORM = 0x8F95
    Rg8Snorm = 36757,
    //
    // Summary:
    //     Original was GL_RGB8_SNORM = 0x8F96
    Rgb8Snorm = 36758,
    //
    // Summary:
    //     [requires: EXT_render_snorm] Original was GL_RGBA8_SNORM = 0x8F97
    Rgba8Snorm = 36759,
    //
    // Summary:
    //     Original was GL_RGB10_A2UI = 0x906F
    Rgb10A2ui = 36975,
}
