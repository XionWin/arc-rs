use std::borrow::{Borrow, BorrowMut};

pub struct GraphicShape {
    _shape: Box<dyn core::Shape>,
    _cache: Option<crate::TextureCache>,
}

impl GraphicShape {
    pub fn get_shape_mut(&mut self) -> &mut dyn core::Shape {
        self._shape.borrow_mut()
    }
    pub fn get_shape(&self) -> &dyn core::Shape {
        self._shape.borrow()
    }
    pub fn get_cache(&self) -> Option<&crate::TextureCache> {
        match &self._cache {
            Some(cache) => Some(cache),
            None => None,
        }
    }
    pub fn set_cache(&mut self, cache: Option<crate::TextureCache>) {
        self._cache = cache
    }
}

impl From<Box<dyn core::Shape>> for GraphicShape {
    fn from(value: Box<dyn core::Shape>) -> Self {
        Self {
            _shape: value,
            _cache: None,
        }
    }
}
