#[repr(u32)]
pub enum ReadBufferMode {
    //
    // Summary:
    //     [requires: KHR_context_flush_control] Original was GL_NONE = 0
    None = 0,
    //
    // Summary:
    //     Original was GL_FRONT_LEFT = 0x0400
    FrontLeft = 1024,
    //
    // Summary:
    //     Original was GL_FRONT_RIGHT = 0x0401
    FrontRight = 1025,
    //
    // Summary:
    //     Original was GL_BACK_LEFT = 0x0402
    BackLeft = 1026,
    //
    // Summary:
    //     Original was GL_BACK_RIGHT = 0x0403
    BackRight = 1027,
    //
    // Summary:
    //     Original was GL_FRONT = 0x0404
    Front = 1028,
    //
    // Summary:
    //     Original was GL_BACK = 0x0405
    Back = 1029,
    //
    // Summary:
    //     Original was GL_LEFT = 0x0406
    Left = 1030,
    //
    // Summary:
    //     Original was GL_RIGHT = 0x0407
    Right = 1031,
    //
    // Summary:
    //     Original was GL_AUX0 = 0x0409
    Aux0 = 1033,
    //
    // Summary:
    //     Original was GL_AUX1 = 0x040A
    Aux1 = 1034,
    //
    // Summary:
    //     Original was GL_AUX2 = 0x040B
    Aux2 = 1035,
    //
    // Summary:
    //     Original was GL_AUX3 = 0x040C
    Aux3 = 1036,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT0 = 0x8CE0
    ColorAttachment0 = 36064,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT1 = 0x8CE1
    ColorAttachment1 = 36065,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT2 = 0x8CE2
    ColorAttachment2 = 36066,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT3 = 0x8CE3
    ColorAttachment3 = 36067,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT4 = 0x8CE4
    ColorAttachment4 = 36068,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT5 = 0x8CE5
    ColorAttachment5 = 36069,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT6 = 0x8CE6
    ColorAttachment6 = 36070,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT7 = 0x8CE7
    ColorAttachment7 = 36071,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT8 = 0x8CE8
    ColorAttachment8 = 36072,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT9 = 0x8CE9
    ColorAttachment9 = 36073,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT10 = 0x8CEA
    ColorAttachment10 = 36074,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT11 = 0x8CEB
    ColorAttachment11 = 36075,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT12 = 0x8CEC
    ColorAttachment12 = 36076,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT13 = 0x8CED
    ColorAttachment13 = 36077,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT14 = 0x8CEE
    ColorAttachment14 = 36078,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT15 = 0x8CEF
    ColorAttachment15 = 36079,
}
