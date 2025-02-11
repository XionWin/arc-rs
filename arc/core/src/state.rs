use std::fmt::Debug;

pub trait State: crate::AsAny + Debug {
    fn get_paint(&self) -> &crate::Paint;
    fn get_transform(&self) -> &crate::Matrix2D;
    fn get_scissor(&self) -> Option<&crate::Scissor>;
}

impl dyn State {
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: State,
    {
        self.as_any().downcast_ref::<T>()
    }
}
