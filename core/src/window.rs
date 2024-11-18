pub trait Window
{
    fn get_graphic(&self) -> &dyn crate::Graphic;
    fn run(&mut self, on_load: fn(&Self), on_render: fn(&Self));
    
    fn set_vsync(&mut self, is_vsync: bool);

    fn init(&mut self);
    fn frame_init(&mut self);

    fn swap_buffers(&self);
}
