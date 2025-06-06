use std::ffi::{c_int, c_void, CString};

pub struct LibraryLoader {
    _handle: *const c_void,
}

#[repr(i32)]
#[allow(dead_code)]
pub(crate) enum OpenFlag {
    RtldLazy = 0x0001,        /* Lazy function call binding.  */
    RtldNow = 0x0002,         /* Immediate function call binding.  */
    RtldBindingMask = 0x0003, /* Mask of binding time value.  */
    RtldNoload = 0x00008,     /* Do not load the object.  */
    RtldDeepbind = 0x00010,   /* Use deep binding.  */
}

impl Into<c_int> for OpenFlag {
    fn into(self) -> c_int {
        self as c_int
    }
}

impl LibraryLoader {
    pub fn new(library_name: &str) -> Self {
        unsafe {
            let cstr = CString::new(library_name).unwrap();
            let ptr = cstr.as_ptr();
            let handle = libc::dlopen(ptr, OpenFlag::RtldNow.into());
            Self { _handle: handle }
        }
    }

    pub fn get_proc_address(&self, function_name: &str) -> *const c_void {
        unsafe {
            let cstr = CString::new(function_name).unwrap();
            let ptr = cstr.as_ptr();
            libc::dlsym(self._handle as _, ptr)
        }
    }
}
