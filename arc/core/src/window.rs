pub trait Window
{
    fn get_graphic(&self) -> &dyn crate::Graphic;
    fn run<T1, T2>(&mut self, on_load: T1, on_render: T2)
    where T1: Fn(&Self), T2: Fn(&Self);
    
    fn set_vsync(&mut self, is_vsync: bool);

    fn init(&mut self);
    fn frame_init(&mut self);

    fn swap_buffers(&self);
}
