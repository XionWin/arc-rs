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
