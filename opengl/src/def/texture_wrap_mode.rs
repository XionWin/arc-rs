#[repr(u32)]
pub enum TextureWrapMode
{
    Clamp = 10496,
    Repeat = 10497,
    ClampToBorder = 33069,
    ClampToEdge = 33071,
}

#[allow(non_upper_case_globals)]
impl TextureWrapMode {
    pub const ClampToBorderArb: TextureWrapMode = TextureWrapMode::ClampToBorder;
    pub const ClampToBorderNv: TextureWrapMode = TextureWrapMode::ClampToBorder;
    pub const ClampToBorderSgis: TextureWrapMode = TextureWrapMode::ClampToBorder;
    pub const ClampToEdgeSgis: TextureWrapMode = TextureWrapMode::ClampToEdge;
    
}