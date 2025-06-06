use std::{borrow::Borrow, fmt::Debug, rc::Rc};

use crate::PaintTexture;

#[derive(Debug)]
pub struct Style {
    background: Box<dyn Background>,
    stroke: ColorBackground,
    stroke_width: Option<i32>,
}

impl Style {
    pub fn new(
        background: Box<dyn Background>,
        stroke: ColorBackground,
        stroke_width: Option<i32>,
    ) -> Self {
        Self {
            background,
            stroke,
            stroke_width,
        }
    }
    pub fn get_background(&self) -> &dyn Background {
        self.background.borrow()
    }
    pub fn get_stroke(&self) -> &ColorBackground {
        &self.stroke
    }
    pub fn get_stroke_width(&self) -> Option<i32> {
        self.stroke_width
    }
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
    paint_color: crate::PaintColor,
}

impl Background for ColorBackground {}

impl Default for ColorBackground {
    fn default() -> Self {
        Self {
            paint_color: crate::PaintColor::default(),
        }
    }
}

impl ColorBackground {
    pub fn new(inner_color: crate::Color, outer_color: crate::Color) -> Self {
        Self {
            paint_color: crate::PaintColor::new(inner_color, outer_color),
        }
    }
    pub fn get_paint_color(&self) -> crate::PaintColor {
        self.paint_color.clone()
    }
}

impl crate::AsAny for ColorBackground {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn box_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

#[derive(Debug)]
pub struct TextureBackground {
    paint_texture: Rc<PaintTexture>,
    paint_color: crate::PaintColor,
}

impl Background for TextureBackground {}

impl TextureBackground {
    pub fn new(paint_texture: Rc<PaintTexture>) -> Self {
        Self {
            paint_texture,
            paint_color: crate::PaintColor::new_with_inner_color(crate::Color::TrueColor {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            }),
        }
    }

    pub fn new_with_paint_color(
        paint_texture: Rc<PaintTexture>,
        paint_color: crate::PaintColor,
    ) -> Self {
        Self {
            paint_texture,
            paint_color,
        }
    }
    pub fn get_paint_texture_rc(&self) -> Rc<PaintTexture> {
        self.paint_texture.clone()
    }
    pub fn get_paint_color(&self) -> crate::PaintColor {
        self.paint_color.clone()
    }
}

impl crate::AsAny for TextureBackground {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn box_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
