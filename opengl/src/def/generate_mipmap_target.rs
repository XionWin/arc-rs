#[repr(u32)]
pub enum GenerateMipmapTarget {
    Texture1D = gl::TEXTURE_1D,
    Texture2D = gl::TEXTURE_2D,
    Texture3D = gl::TEXTURE_3D,
    TextureCubeMap = gl::TEXTURE_CUBE_MAP,
    Texture1DArray = gl::TEXTURE_1D_ARRAY,
    Texture2DArray = gl::TEXTURE_2D_ARRAY,
    TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
    Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY
}