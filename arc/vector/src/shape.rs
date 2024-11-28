pub trait Shape {
    fn get_stroke_primitive(&self) -> crate::Primitive;
    fn get_fill_primitive(&self) -> crate::Primitive;
}

impl<T: core::Shape + ?Sized> Shape for T {
    fn get_stroke_primitive(&self) -> crate::Primitive {
        todo!()
    }

    fn get_fill_primitive(&self) -> crate::Primitive {
        todo!()
    }
}
