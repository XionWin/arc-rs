use bitflags::bitflags;

use std::ffi::c_uint;

bitflags! {
    #[derive(Default)]
        pub struct BlendFactor: c_uint
        {
            const ZERO = 1 << 0;
            const ONE = 1 << 1;
            const SRC_COLOR = 1 << 2;
            const ONE_MINUS_SRC_COLOR = 1 << 3;
            const DST_COLOR = 1 << 4;
            const ONE_MINUS_DST_COLOR = 1 << 5;
            const SRC_ALPHA = 1 << 6;
            const ONE_MINUS_SRC_ALPHA = 1 << 7;
            const DST_ALPHA = 1 << 8;
            const ONE_MINUS_DST_ALPHA = 1 << 9;
            const SRC_ALPHA_SATURATE = 1 << 10;
        }
}
