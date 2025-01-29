use vector::VectorShape;

use crate::{Cacheable, Container};

pub trait Drawable: Cacheable + VectorShape {
    fn get_container(&self) -> Option<&dyn Container>;
}
