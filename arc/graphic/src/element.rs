use crate::{Container, GraphicShape, TextureCache};

pub struct Element {
    _graphic_shape: Option<GraphicShape>,
    _container: Option<Container>,
    _cache: Option<crate::TextureCache>,
}

impl Element {
    pub fn get_graphic_shape_mut(&mut self) -> Option<&mut GraphicShape> {
        match &mut self._graphic_shape {
            Some(graphic_shape) => Some(graphic_shape),
            None => None,
        }
    }
    pub fn get_graphic_shape(&self) -> Option<&GraphicShape> {
        self._graphic_shape.as_ref()
    }
    pub fn get_cache(&self) -> Option<&TextureCache> {
        self._cache.as_ref()
    }
    pub fn get_container(&self) -> Option<&Container> {
        self._container.as_ref()
    }

    pub fn set_cache(&mut self, cache: TextureCache) {
        self._cache = Some(cache)
    }
}

impl From<Box<dyn core::Shape>> for Element {
    fn from(value: Box<dyn core::Shape>) -> Self {
        Self {
            _graphic_shape: Some(value.into()),
            _container: None,
            _cache: None,
        }
    }
}

impl From<Container> for Element {
    fn from(value: Container) -> Self {
        Self {
            _graphic_shape: None,
            _container: Some(value),
            _cache: None,
        }
    }
}
