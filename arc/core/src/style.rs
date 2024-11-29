use crate::{Color, Image, Matrix2D};

pub struct Style {
    pub background: Box<dyn Background>,
    pub stroke: ColorPaint,
    pub stroke_width: Option<i32>,
}

pub trait Background {}

pub struct ColorPaint {
    inner_color: Color,
    outer_color: Color,
}

pub struct ImagePaint {
    image: Box<dyn Image>,
    transform: Matrix2D,
}
