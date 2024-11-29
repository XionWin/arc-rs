use crate::Point;

#[derive(Debug, PartialEq)]
pub enum Command {
    MoveTo(Point<f32>),
    LineTo(Point<f32>),
    BezierTo(Point<f32>, Point<f32>, Point<f32>),
    Close,
}
