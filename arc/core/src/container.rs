use std::fmt::Debug;

use crate::AsAny;

pub trait Container: Debug + AsAny {
    fn add(&mut self, shape: Box<dyn crate::Shape>);
    fn get_children(&self) -> Vec<&dyn crate::Shape>;
}
