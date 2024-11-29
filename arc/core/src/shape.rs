pub trait Shape {
    fn get_shape(&self) -> &dyn Shape;
}
