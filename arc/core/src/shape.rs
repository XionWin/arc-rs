use std::fmt::Debug;

use crate::{Style, Vertex2};

pub trait Shape: Debug {
    fn get_shape_ref(&self) -> &dyn Shape;

    fn get_vertexes(&self) -> &[Vertex2];
    fn get_style(&self) -> &Style;
}
