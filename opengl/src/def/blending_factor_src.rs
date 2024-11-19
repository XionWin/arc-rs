#[repr(u32)]
pub enum BlendingFactorSrc
{
    //
    // Summary:
    //     Original was GL_Zero = 0
    Zero = 0,
    //
    // Summary:
    //     Original was GL_SrcColor = 0X0300
    SrcColor = 768,
    //
    // Summary:
    //     Original was GL_OneMinusSrcColor = 0X0301
    OneMinusSrcColor = 769,
    //
    // Summary:
    //     Original was GL_SrcAlpha = 0X0302
    SrcAlpha = 770,
    //
    // Summary:
    //     Original was GL_OneMinusSrcAlpha = 0X0303
    OneMinusSrcAlpha = 771,
    //
    // Summary:
    //     Original was GL_DstAlpha = 0X0304
    DstAlpha = 772,
    //
    // Summary:
    //     Original was GL_OneMinusDstAlpha = 0X0305
    OneMinusDstAlpha = 773,
    //
    // Summary:
    //     Original was GL_DstColor = 0X0306
    DstColor = 774,
    //
    // Summary:
    //     Original was GL_OneMinusDstColor = 0X0307
    OneMinusDstColor = 775,
    //
    // Summary:
    //     Original was GL_SrcAlphaSaturate = 0X0308
    SrcAlphaSaturate = 776,
    //
    // Summary:
    //     Original was GL_ConstantColor = 0X8001
    ConstantColor = 32769,
    //
    // Summary:
    //     Original was GL_OneMinusConstantColor = 0X8002
    OneMinusConstantColor = 32770,
    //
    // Summary:
    //     Original was GL_ConstantAlpha = 0X8003
    ConstantAlpha = 32771,
    //
    // Summary:
    //     Original was GL_OneMinusConstantAlpha = 0X8004
    OneMinusConstantAlpha = 32772,
    //
    // Summary:
    //     Original was GL_One = 1
    One = 1
}