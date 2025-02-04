use core::Container;
use std::borrow::Borrow;

use crate::{GraphicShape, TextureCache};

pub struct Element {
    _is_enabled_cache: bool,
    _graphic_shape: Option<GraphicShape>,
    _container: Option<Box<dyn core::Container>>,
    _cache: Option<crate::TextureCache>,
}

impl Element {
    pub fn get_is_enabled_cache(&self) -> bool {
        self._is_enabled_cache
    }
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
        match self._is_enabled_cache {
            true => self._cache.as_ref(),
            false => None,
        }
    }
    pub fn get_container(&self) -> Option<&dyn Container> {
        match &self._container {
            Some(container) => Some(container.borrow()),
            None => None,
        }
    }

    pub fn set_cache(&mut self, cache: TextureCache) {
        match self._is_enabled_cache {
            true => self._cache = Some(cache),
            false => util::print_panic!("attempting to modify a non-updatable element"),
        }
    }
}

impl From<Box<dyn core::Shape>> for Element {
    fn from(value: Box<dyn core::Shape>) -> Self {
        Self {
            _is_enabled_cache: true,
            _graphic_shape: Some(value.into()),
            _container: None,
            _cache: None,
        }
    }
}

impl From<Box<dyn core::Container>> for Element {
    fn from(value: Box<dyn core::Container>) -> Self {
        let container = value.box_any().downcast::<crate::Container>().unwrap();
        Self {
            _is_enabled_cache: true,
            _graphic_shape: None,
            _container: Some(container),
            _cache: None,
        }
    }
}
