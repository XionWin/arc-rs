pub trait Container {
    fn add(&mut self, shape: Box<dyn crate::Shape>);
    fn get_children(&self) -> Vec<&dyn crate::Shape>;
}
