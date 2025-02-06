use std::rc::Rc;

use crate::{PaintColor, PaintTexture};

#[derive(Debug)]
pub struct Paint {
    _paint_texture: Option<Rc<PaintTexture>>,
    _paint_color: PaintColor,
    _radius: f32,
    _feather: f32,
    _alpha: f32,
}

impl Paint {
    pub fn new_from_texture(
        texture: Rc<dyn crate::Texture>,
        view_port: crate::Rectangle<i32>,
    ) -> Self {
        Self {
            _paint_texture: Some(Rc::new(PaintTexture::new(texture, view_port))),
            _paint_color: PaintColor::default(),
            _radius: 0f32,
            _feather: 1f32,
            _alpha: 1f32,
        }
    }
    pub fn try_get_paint_texture(&self) -> Option<&PaintTexture> {
        match &self._paint_texture {
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
            _paint_texture: Option::None,
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
                _paint_texture: Option::None,
                _paint_color: color_background.get_paint_color(),
                _radius: 0f32,
                _feather: 0f32,
                _alpha: 0f32,
            },
            None => match value.downcast_ref::<crate::ImageBackground>() {
                Some(image_background) => Self {
                    _paint_texture: Some(image_background.get_paint_image_rc()),
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
