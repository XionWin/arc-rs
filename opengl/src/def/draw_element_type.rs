
#[repr(u32)]
pub enum DrawElementsType
{
    //
    // Summary:
    //     Original was GL_UNSIGNED_BYTE = 0x1401
    UnsignedByte = 5121,
    //
    // Summary:
    //     [requires: ANGLE_depth_texture, or OES_depth_texture] Original was GL_UNSIGNED_SHORT
    //     = 0x1403
    UnsignedShort = 5123,
    //
    // Summary:
    //     [requires: ANGLE_depth_texture, or OES_depth_texture, OES_element_index_uint]
    //     Original was GL_UNSIGNED_INT = 0x1405
    UnsignedInt = 5125
}