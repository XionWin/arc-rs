use crate::color::Color;

pub trait Graphic {
    fn viewport(x: i32, y: i32, width: i32, height: i32);
    fn clear_color(color: Color);
}