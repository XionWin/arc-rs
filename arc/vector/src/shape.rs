pub trait Shape {
    fn get_stroke_primitive(&self) -> &crate::Primitive;
    fn get_fill_primitive(&self) -> &crate::Primitive;
}
