use std::borrow::Borrow;

pub struct GraphicShape {
    _shape: Box<dyn core::Shape>,
    _cache: Option<crate::ShapeCache>,
}

// impl<T: core::Shape + Sized> From<T> for GraphicShape {
//     fn from(value: T) -> Self {
//         Self {
//             shape: Box::new(value),
//             cache: None,
//         }
//     }
// }

impl GraphicShape {
    pub fn get_shape(&self) -> &dyn core::Shape {
        self._shape.borrow()
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
