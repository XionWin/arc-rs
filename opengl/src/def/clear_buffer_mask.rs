use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
        pub struct ClearBufferMask: u32 {
        const COLOR_BUFFER_BIT = gl::COLOR_BUFFER_BIT;
        const DEPTH_BUFFER_BIT = gl::DEPTH_BUFFER_BIT;
    }
}

impl Into<u32> for ClearBufferMask {
    fn into(self) -> u32 {
        self.bits
    }
}
