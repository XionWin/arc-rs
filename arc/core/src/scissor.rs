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

impl Scissor {
    pub fn get_transform(&self) -> &crate::Matrix2D {
        &self._transform
    }

    pub fn get_extent(&self) -> crate::Extent<f32> {
        self._extent
    }
}
