pub struct Scissor {
    _transform: crate::Matrix2D,
    _extent: crate::Extent<f32>,
}

impl Default for Scissor {
    fn default() -> Self {
        Self {
            _transform: crate::Matrix2D::default(),
            _extent: crate::Extent::default(),
        }
    }
}
