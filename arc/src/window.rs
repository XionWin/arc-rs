pub trait Window
{
    fn gl_get_proc_address(&self, procname: &str) -> *const std::ffi::c_void;
}
