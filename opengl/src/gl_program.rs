use std::ffi::c_uint;

pub trait GLProgram {
    fn get_id(&self) -> c_uint;
}
