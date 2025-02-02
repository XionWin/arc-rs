pub trait Container {
    fn add(&mut self, shape: Box<dyn core::Shape>);
    fn get_children(&self) -> Vec<&dyn core::Shape>;
    fn update_cache(&mut self, cache: crate::TextureCache);
    fn get_cache(&self) -> &crate::TextureCache;
}
