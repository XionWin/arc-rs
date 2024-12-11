use std::{fmt::Debug, rc::Rc};

use crate::{Color, Matrix2D, PaintImage};

#[derive(Debug)]
pub struct Style {
    pub background: Box<dyn Background>,
    pub stroke: ColorBackground,
    pub stroke_width: Option<i32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Box::new(ColorBackground::default()),
            stroke: ColorBackground::default(),
            stroke_width: None,
        }
    }
}

pub trait Background: Debug + crate::AsAny {}

impl dyn Background {
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Background,
    {
        self.as_any().downcast_ref::<T>()
    }
}

#[derive(Debug)]
pub struct ColorBackground {
    _paint_color: crate::PaintColor,
}

impl Background for ColorBackground {}

impl Default for ColorBackground {
    fn default() -> Self {
        Self {
            _paint_color: crate::PaintColor::default(),
        }
    }
}

impl ColorBackground {
    pub fn new(inner_color: crate::Color, outer_color: crate::Color) -> Self {
        Self {
            _paint_color: crate::PaintColor::new(inner_color, outer_color),
        }
    }
    pub fn get_inner_color(&self) -> Color {
        self._paint_color.get_inner_color()
    }
    pub fn get_outer_color(&self) -> Color {
        self._paint_color.get_outer_color()
    }
    pub fn get_paint_color(&self) -> crate::PaintColor {
        self._paint_color.clone()
    }
}

impl crate::AsAny for ColorBackground {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct ImageBackground {
    image: Rc<PaintImage>,
    _paint_color: crate::PaintColor,
    transform: Matrix2D,
}

impl Background for ImageBackground {}

impl ImageBackground {
    pub fn get_paint_image_rc(&self) -> Rc<PaintImage> {
        self.image.clone()
    }
    pub fn get_paint_color(&self) -> crate::PaintColor {
        self._paint_color.clone()
    }
    pub fn get_transform(&self) -> &Matrix2D {
        &self.transform
    }
}

impl crate::AsAny for ImageBackground {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
