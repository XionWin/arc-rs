use crate::Drawable;

pub trait Container: Drawable {
    fn add(&mut self, shape: Box<dyn core::Shape>);
    fn get_children(&self) -> Vec<&dyn core::Shape>;
}
