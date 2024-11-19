pub enum TextureTarget {
    Texture1D = 3552,
    Texture2D = 3553,
    ProxyTexture1D = 32867,
    ProxyTexture2D = 32868,
    Texture3D = 32879,
    ProxyTexture3D = 32880,
    DetailTexture2DSgis = 32917,
    Texture4DSgis = 33076,
    ProxyTexture4DSgis = 33077,
    TextureRectangle = 34037,
    ProxyTextureRectangle = 34039,
    TextureCubeMap = 34067,
    TextureCubeMapPositiveX = 34069,
    TextureCubeMapNegativeX = 34070,
    TextureCubeMapPositiveY = 34071,
    TextureCubeMapNegativeY = 34072,
    TextureCubeMapPositiveZ = 34073,
    TextureCubeMapNegativeZ = 34074,
    ProxyTextureCubeMap = 34075,
    Texture1DArray = 35864,
    ProxyTexture1DArray = 35865,
    Texture2DArray = 35866,
    ProxyTexture2DArray = 35867,
    TextureCubeMapArray = 36873,
    ProxyTextureCubeMapArray = 36875,
    Texture2DMultisample = 37120,
    ProxyTexture2DMultisample = 37121,
    Texture2DMultisampleArray = 37122,
    ProxyTexture2DMultisampleArray = 37123
}

#[allow(non_upper_case_globals)]
impl TextureTarget {
    pub const ProxyTexture1DExt: TextureTarget = TextureTarget::ProxyTexture1D;
    pub const ProxyTexture2DExt: TextureTarget = TextureTarget::ProxyTexture2D;
    pub const Texture3DExt: TextureTarget = TextureTarget::Texture3D;
    pub const Texture3DOes: TextureTarget = TextureTarget::Texture3D;
    pub const ProxyTexture3DExt: TextureTarget = TextureTarget::ProxyTexture3D;
    pub const ProxyTextureRectangleArb: TextureTarget = TextureTarget::ProxyTextureRectangle;
    pub const ProxyTextureRectangleNv: TextureTarget = TextureTarget::ProxyTextureRectangle;
    pub const ProxyTextureCubeMapArb: TextureTarget = TextureTarget::ProxyTextureCubeMap;
    pub const ProxyTextureCubeMapExt: TextureTarget = TextureTarget::ProxyTextureCubeMap;
    pub const ProxyTexture1DArrayExt: TextureTarget = TextureTarget::ProxyTexture1DArray;
    pub const ProxyTexture2DArrayExt: TextureTarget = TextureTarget::ProxyTexture2DArray;
    pub const TextureCubeMapArrayArb: TextureTarget = TextureTarget::TextureCubeMapArray;
    pub const TextureCubeMapArrayExt: TextureTarget = TextureTarget::TextureCubeMapArray;
    pub const TextureCubeMapArrayOes: TextureTarget = TextureTarget::TextureCubeMapArray;
    pub const ProxyTextureCubeMapArrayArb: TextureTarget = TextureTarget::ProxyTextureCubeMapArray;
}