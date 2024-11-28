use std::rc::Rc;

use crate::{PaintColor, PaintImage};

#[derive(Debug)]
pub struct Paint {
    _image: Option<PaintImage>,
    _color: PaintColor,
    _radius: f32,
    _feather: f32,
    _alpha: f32,
}

impl Paint {
    pub fn new_from_image(image: Rc<dyn crate::Image>, view_port: crate::Rect<i32>) -> Self {
        Self {
            _image: Some(PaintImage::new(image, view_port)),
            _color: PaintColor::default(),
            _radius: 0f32,
            _feather: 1f32,
            _alpha: 1f32,
        }
    }
    pub fn get_image(&self) -> Option<&PaintImage> {
        match &self._image {
            Some(x) => Some(x),
            None => None,
        }
    }
    pub fn get_color(&self) -> &PaintColor {
        &self._color
    }
    pub fn get_radius(&self) -> f32 {
        self._radius
    }
    pub fn get_feather(&self) -> f32 {
        self._feather
    }
    pub fn get_alpha(&self) -> f32 {
        self._alpha
    }
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            _image: Option::None,
            _color: PaintColor::default(),
            _radius: 0f32,
            _feather: 1f32,
            _alpha: 1f32,
        }
    }
}
