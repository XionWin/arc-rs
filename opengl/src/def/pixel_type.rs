pub enum PixelType
{
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    Int = 5124,
    UnsignedInt = 5125,
    Float = 5126,
    Bitmap = 6656,
    UnsignedByte332 = 32818,
    UnsignedShort4444 = 32819,
    UnsignedShort5551 = 32820,
    UnsignedInt8888 = 32821,
    UnsignedInt1010102 = 32822,
    UnsignedShort565 = 33635
}

#[allow(non_upper_case_globals)]
impl PixelType {
    pub const UnsignedByte332Ext: PixelType = PixelType::UnsignedByte332;
    pub const UnsignedShort4444Ext: PixelType = PixelType::UnsignedShort4444;
    pub const UnsignedShort5551Ext: PixelType = PixelType::UnsignedShort5551;
    pub const UnsignedInt8888Ext: PixelType = PixelType::UnsignedInt8888;
    pub const UnsignedInt1010102Ext: PixelType = PixelType::UnsignedInt1010102;
    
}