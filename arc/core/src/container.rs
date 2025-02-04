use std::fmt::Debug;

use crate::AsAny;

pub trait Container: Debug + AsAny {
    fn add_shape(&mut self, shape: Box<dyn crate::Shape>);
    fn add_container(&mut self, container: Box<dyn crate::Container>);
}
