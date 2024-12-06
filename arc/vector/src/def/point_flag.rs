use std::ffi::c_uint;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct PointFlag: c_uint {
        const NONE = 0 << 0;
        const CORNER = 1 << 0;
        const LEFT = 1 << 1;
        const BEVEL = 1 << 2;
        const INNER_BEVEL = 1 << 3;
    }
}
