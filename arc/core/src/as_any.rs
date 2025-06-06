use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn box_any(self: Box<Self>) -> Box<dyn Any>;
}
