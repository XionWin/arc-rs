use std::{borrow::Borrow, cell::RefCell};

use crate::{Container, GraphicShape, TextureCache};

pub struct DrawElement {
    _graphic_shape: Option<GraphicShape>,
    _cache: Option<RefCell<TextureCache>>,
    _container: Option<Box<dyn Container>>,
}

impl DrawElement {
    pub fn get_graphic_shape(&self) -> Option<&GraphicShape> {
        self._graphic_shape.as_ref()
    }
    pub fn get_cache(&self) -> Option<&RefCell<TextureCache>> {
        self._cache.as_ref()
    }
    pub fn get_container(&self) -> Option<&dyn Container> {
        match &self._container {
            Some(container) => Some(container.borrow()),
            None => None,
        }
    }
}

impl From<Box<dyn core::Shape>> for DrawElement {
    fn from(value: Box<dyn core::Shape>) -> Self {
        Self {
            _graphic_shape: Some(value.into()),
            _cache: None,
            _container: None,
        }
    }
}
