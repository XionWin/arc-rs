use std::rc::Rc;

use crate::{PaintColor, PaintImage};

#[derive(Debug)]
pub struct Paint {
    _paint_image: Option<Rc<PaintImage>>,
    _paint_color: PaintColor,
    _radius: f32,
    _feather: f32,
    _alpha: f32,
}

impl Paint {
    pub fn new_from_image(image: Rc<dyn crate::Image>, view_port: crate::Rect<i32>) -> Self {
        Self {
            _paint_image: Some(Rc::new(PaintImage::new(image, view_port))),
            _paint_color: PaintColor::default(),
            _radius: 0f32,
            _feather: 1f32,
            _alpha: 1f32,
        }
    }
    pub fn try_get_paint_image(&self) -> Option<&PaintImage> {
        match &self._paint_image {
            Some(x) => Some(x),
            None => None,
        }
    }
    pub fn get_color(&self) -> &PaintColor {
        &self._paint_color
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
            _paint_image: Option::None,
            _paint_color: PaintColor::default(),
            _radius: 0f32,
            _feather: 1f32,
            _alpha: 1f32,
        }
    }
}

impl From<&dyn crate::Background> for Paint {
    fn from(value: &dyn crate::Background) -> Self {
        match value.downcast_ref::<crate::ColorBackground>() {
            Some(color_background) => Self {
                _paint_image: Option::None,
                _paint_color: color_background.get_paint_color(),
                _radius: 0f32,
                _feather: 0f32,
                _alpha: 0f32,
            },
            None => match value.downcast_ref::<crate::ImageBackground>() {
                Some(image_background) => Self {
                    _paint_image: Some(image_background.get_paint_image_rc()),
                    _paint_color: image_background.get_paint_color(),
                    _radius: 0f32,
                    _feather: 0f32,
                    _alpha: 0f32,
                },
                None => util::print_panic!("paint from convert error"),
            },
        }
    }
}
