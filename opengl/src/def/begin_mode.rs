#[repr(u32)]
pub enum BeginMode {
    //
    // Summary:
    //     Original was GL_Points = 0X0000
    Points,
    //
    // Summary:
    //     Original was GL_Lines = 0X0001
    Lines,
    //
    // Summary:
    //     Original was GL_LineLoop = 0X0002
    LineLoop,
    //
    // Summary:
    //     Original was GL_LineStrip = 0X0003
    LineStrip,
    //
    // Summary:
    //     Original was GL_Triangles = 0X0004
    Triangles,
    //
    // Summary:
    //     Original was GL_TriangleStrip = 0X0005
    TriangleStrip,
    //
    // Summary:
    //     Original was GL_TriangleFan = 0X0006
    TriangleFan
}