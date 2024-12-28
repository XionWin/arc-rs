#[derive(Debug)]
pub struct Scissor {
    transform: crate::Matrix2D,
    extent: crate::Extent<i32>,
}

impl Default for Scissor {
    fn default() -> Self {
        Self {
            transform: crate::Matrix2D::default(),
            extent: crate::Extent::default(),
        }
    }
}

impl Scissor {
    pub fn new(transform: crate::Matrix2D, extent: crate::Extent<i32>) -> Self {
        Self { transform, extent }
    }
    pub fn get_transform(&self) -> &crate::Matrix2D {
        &self.transform
    }

    pub fn get_extent(&self) -> crate::Extent<i32> {
        self.extent
    }
}
