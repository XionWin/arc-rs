use std::rc::Rc;

use crate::{Matrix, Matrix2D};

pub struct Paint {
    _rect: crate::Rect<i32>,
    _transform: Box<dyn Matrix<f32>>,
    _extent: crate::Extent<f32>,
    _radius: f32,
    _feather: f32,
    _inner_color: crate::Color,
    _outer_color: crate::Color,
    _image: Rc<dyn crate::Image>,
}

impl Paint {
    pub fn new(image: Rc<dyn crate::Image>, view: crate::Rect<i32>) -> Self {
        Self {
            _rect: view,
            _transform: Box::new(Matrix2D::<f32>::new()),
            _extent: crate::Extent::new(view.size.width as _, view.size.height as _),
            _radius: 0f32,
            _feather: 1f32,
            _inner_color: crate::Color::Transparent,
            _outer_color: crate::Color::Transparent,
            _image: image,
        }
    }
}
