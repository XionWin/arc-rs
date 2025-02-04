use std::borrow::{Borrow, BorrowMut};

pub struct GraphicShape {
    _shape: Box<dyn core::Shape>,
    // _cache: Option<crate::TextureCache>,
}

impl GraphicShape {
    pub fn get_shape_mut(&mut self) -> &mut dyn core::Shape {
        self._shape.borrow_mut()
    }
    pub fn get_shape(&self) -> &dyn core::Shape {
        self._shape.borrow()
    }
}

impl From<Box<dyn core::Shape>> for GraphicShape {
    fn from(value: Box<dyn core::Shape>) -> Self {
        Self {
            _shape: value,
            // _cache: None,
        }
    }
}
