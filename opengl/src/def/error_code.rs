#[repr(u32)]
#[derive(Debug)]
pub enum ErrorCode {
    //
    // Summary:
    //     [requires: EXT_robustness, or KHR_robustness] Original was GL_NO_ERROR = 0
    NoError = 0,
    //
    // Summary:
    //     Original was GL_INVALID_ENUM = 0x0500
    InvalidEnum = 1280,
    //
    // Summary:
    //     Original was GL_INVALID_VALUE = 0x0501
    InvalidValue = 1281,
    //
    // Summary:
    //     Original was GL_INVALID_OPERATION = 0x0502
    InvalidOperation = 1282,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_STACK_OVERFLOW = 0x0503
    StackOverflow = 1283,
    //
    // Summary:
    //     [requires: KHR_debug] Original was GL_STACK_UNDERFLOW = 0x0504
    StackUnderflow = 1284,
    //
    // Summary:
    //     Original was GL_OUT_OF_MEMORY = 0x0505
    OutOfMemory = 1285,
    //
    // Summary:
    //     Original was GL_INVALID_FRAMEBUFFER_OPERATION = 0x0506
    InvalidFramebufferOperation = 1286,
    //
    // Summary:
    //     [requires: KHR_robustness] Original was GL_CONTEXT_LOST = 0x0507
    ContextLost = 1287,
    //
    // Summary:
    //     Original was GL_TABLE_TOO_LARGE = 0x8031
    TableTooLarge = 32817,
    //
    // Summary:
    //     Original was GL_TEXTURE_TOO_LARGE_EXT = 0x8065
    TextureTooLargeExt = 32869,
}

#[allow(non_upper_case_globals)]
impl ErrorCode {
    pub const InvalidFramebufferOperationExt: ErrorCode = ErrorCode::InvalidFramebufferOperation;
    pub const InvalidFramebufferOperationOes: ErrorCode = ErrorCode::InvalidFramebufferOperation;
    pub const TableTooLargeExt: ErrorCode = ErrorCode::TableTooLarge;
}
