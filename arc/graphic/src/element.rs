use std::borrow::Borrow;

use crate::{Container, GraphicShape, TextureCache};

pub struct Element {
    _is_enabled_cache: bool,
    _graphic_shape: Option<GraphicShape>,
    _container: Option<Box<dyn Container>>,
}

impl Element {
    pub fn get_graphic_shape(&self) -> Option<&GraphicShape> {
        self._graphic_shape.as_ref()
    }
    pub fn get_cache(&self) -> Option<&TextureCache> {
        match self._is_enabled_cache {
            true => match &self._container {
                Some(container) => Some(Borrow::<dyn Container>::borrow(container).get_cache()),
                None => None,
            },
            false => None,
        }
    }
    pub fn get_container(&self) -> Option<&dyn Container> {
        match &self._container {
            Some(container) => Some(container.borrow()),
            None => None,
        }
    }

    pub fn update_cache(&mut self, cache: TextureCache) {
        match self._is_enabled_cache {
            true => match &mut self._container {
                Some(container) => container.update_cache(cache),
                None => util::print_panic!("container is null"),
            },
            false => util::print_panic!("attempting to modify a non-updatable element"),
        }
    }
}

impl From<Box<dyn core::Shape>> for Element {
    fn from(value: Box<dyn core::Shape>) -> Self {
        Self {
            _is_enabled_cache: false,
            _graphic_shape: Some(value.into()),
            _container: None,
        }
    }
}
impl From<Box<dyn crate::Container>> for Element {
    fn from(value: Box<dyn crate::Container>) -> Self {
        Self {
            _is_enabled_cache: true,
            _graphic_shape: None,
            _container: Some(value),
        }
    }
}
