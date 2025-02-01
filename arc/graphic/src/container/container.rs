use core::{Rectangle, Size};

pub trait Container {
    fn add(&mut self, shape: Box<dyn core::Shape>);
    fn get_children(&self) -> Vec<&dyn core::Shape>;
    fn get_rectangle(&self) -> Rectangle<i32>;
    fn get_rect(&self) -> core::Rect<i32>;
    fn get_size(&self) -> Size<i32>;
}
