use crate::Graphic;


pub trait Window
{
    fn get_graphic(&self) -> &dyn Graphic;
    fn gl_get_proc_address(&self, procname: &str) -> *const std::ffi::c_void;
    fn run(&mut self, on_load: fn(&Self), on_render: fn(&Self));
    
    fn set_vsync(&mut self, is_vsync: bool);
}
