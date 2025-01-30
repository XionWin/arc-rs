pub trait Container {
    fn add(&mut self, shape: Box<dyn core::Shape>);
    fn get_children(&self) -> Vec<&dyn core::Shape>;
    fn get_rect(&self) -> Option<core::Rect<i32>>;
}
