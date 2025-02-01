use crate::{Container, GraphicShape, TextureCache};

pub struct DrawElement {
    _graphic_shape: Option<GraphicShape>,
    _cache: Option<TextureCache>,
    _container: Option<Box<dyn Container>>,
}

impl DrawElement {
    pub fn get_graphic_shape(&self) -> Option<GraphicShape> {
        todo!()
    }
    pub fn get_cache(&self) -> Option<&TextureCache> {
        todo!()
    }
    pub fn get_container(&self) -> Option<&dyn Container> {
        todo!()
    }
}
