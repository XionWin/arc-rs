use std::ffi::c_uint;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
        pub struct ClearBufferMask: c_uint {
        const COLOR_BUFFER_BIT = gl::COLOR_BUFFER_BIT;
        const DEPTH_BUFFER_BIT = gl::DEPTH_BUFFER_BIT;
    }
}

impl Into<c_uint> for ClearBufferMask {
    fn into(self) -> c_uint {
        self.bits()
    }
}
