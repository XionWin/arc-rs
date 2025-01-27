#[repr(u32)]
#[derive(Debug)]
pub enum FramebufferErrorCode {
    //
    // Summary:
    //     Original was GL_FramebufferComplete = 0X8cd5
    FramebufferComplete = 36053,
    //
    // Summary:
    //     Original was GL_FramebufferIncompleteAttachment = 0X8cd6
    FramebufferIncompleteAttachment = 36054,
    //
    // Summary:
    //     Original was GL_FramebufferIncompleteMissingAttachment = 0X8cd7
    FramebufferIncompleteMissingAttachment = 36055,
    //
    // Summary:
    //     Original was GL_FramebufferIncompleteDimensions = 0X8cd9
    FramebufferIncompleteDimensions = 36057,
    //
    // Summary:
    //     Original was GL_FramebufferUnsupported = 0X8cdd
    FramebufferUnsupported = 36061,
    //
    // Summary:
    //     Original was GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE = 0x8D56
    FramebufferIncompleteMultisample = 36182,
}
