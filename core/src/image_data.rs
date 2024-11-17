use crate::{ColorType, Size};

pub struct ImageData {
    pub size: Size<i32>,
    pub value: Vec<u8>,
    pub color_type: ColorType
}

impl ImageData {
    pub fn new(size: Size<i32>, value: impl Into<Vec<u8>>, color_type: ColorType) -> Self {
        Self {
            size,
            value: value.into(),
            color_type
        }
    }
}