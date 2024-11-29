use std::{borrow::Borrow, fmt::Debug};

use crate::{Color, Image, Matrix2D};

#[derive(Debug)]
pub struct Style {
    pub background: Box<dyn Background>,
    pub stroke: ColorPaint,
    pub stroke_width: Option<i32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Box::new(ColorPaint::default()),
            stroke: ColorPaint::default(),
            stroke_width: None,
        }
    }
}

pub trait Background: Debug {}

#[derive(Debug)]
pub struct ColorPaint {
    inner_color: Color,
    outer_color: Color,
}

impl Background for ColorPaint {}

impl Default for ColorPaint {
    fn default() -> Self {
        Self {
            inner_color: Color::default(),
            outer_color: Default::default(),
        }
    }
}

impl ColorPaint {
    pub fn get_inner_color(&self) -> Color {
        self.inner_color
    }
    pub fn get_outer_color(&self) -> Color {
        self.outer_color
    }
}

#[derive(Debug)]
pub struct ImagePaint {
    image: Box<dyn Image>,
    transform: Matrix2D,
}

impl Background for ImagePaint {}

impl ImagePaint {
    pub fn get_image(&self) -> &dyn Image {
        self.image.borrow()
    }

    pub fn get_transform(&self) -> &Matrix2D {
        &self.transform
    }
}
