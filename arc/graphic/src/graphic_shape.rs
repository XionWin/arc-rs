use std::borrow::Borrow;

pub struct GraphicShape {
    _shape: Box<dyn core::Shape>,
    _stroke_cache: Option<crate::TextureCache>,
    _fill_cache: Option<crate::TextureCache>,
}

impl GraphicShape {
    pub fn get_shape(&self) -> &dyn core::Shape {
        self._shape.borrow()
    }
    pub fn get_fill_cache(&self) -> Option<&crate::TextureCache> {
        match &self._fill_cache {
            Some(cache) => Some(cache),
            None => None,
        }
    }
    pub fn get_stroke_cache(&self) -> Option<&crate::TextureCache> {
        match &self._stroke_cache {
            Some(cache) => Some(cache),
            None => None,
        }
    }
    pub fn set_fill_cache(&mut self, cache: Option<crate::TextureCache>) {
        self._fill_cache = cache
    }
    pub fn set_stroke_cache(&mut self, cache: Option<crate::TextureCache>) {
        self._stroke_cache = cache
    }
}

impl From<Box<dyn core::Shape>> for GraphicShape {
    fn from(value: Box<dyn core::Shape>) -> Self {
        Self {
            _shape: value,
            _stroke_cache: None,
            _fill_cache: None,
        }
    }
}
