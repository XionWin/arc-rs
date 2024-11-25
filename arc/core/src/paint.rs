use std::rc::Rc;

use crate::{Matrix2D, PaintImage};

pub struct Paint {
    _transform: Matrix2D,
    _radius: f32,
    _feather: f32,
    _inner_color: crate::Color,
    _outer_color: crate::Color,
    _image: Option<PaintImage>,
    _alpha: f32,
}

impl Paint {
    pub fn new_from_image(image: Rc<dyn crate::Image>, view_port: crate::Rect<i32>) -> Self {
        Self {
            _transform: Matrix2D::default(),
            _radius: 0f32,
            _feather: 1f32,
            _inner_color: crate::Color::default(),
            _outer_color: crate::Color::default(),
            _image: Some(PaintImage::new(image, view_port)),
            _alpha: 1f32,
        }
    }
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            _transform: Matrix2D::default(),
            _radius: 0f32,
            _feather: 1f32,
            _inner_color: crate::Color::default(),
            _outer_color: crate::Color::default(),
            _image: Option::None,
            _alpha: 1f32,
        }
    }
}
