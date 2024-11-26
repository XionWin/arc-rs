use std::ffi::c_uint;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct PointFlags: c_uint {
        const NONE = 1 << 0;
        const CORNER = 1 << 1;
        const LEFT = 1 << 2;
        const BEVEL = 1 << 3;
        const INNER_BEVEL = 1 << 4;
    }
}
