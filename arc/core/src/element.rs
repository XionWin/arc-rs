pub trait Element {
    fn get_shape(&self) -> Option<&dyn crate::Shape>;
    fn get_container(&self) -> Option<&dyn crate::Container>;
}
