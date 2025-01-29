use core::AsAny;

use vector::VectorShape;

use crate::{Cacheable, Container};

pub trait Drawable: AsAny + Cacheable + VectorShape {
    fn get_container(&self) -> Option<&dyn Container>;
}
