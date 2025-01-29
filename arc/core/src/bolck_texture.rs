use crate::{ImageData, Rectangle};

pub struct Block<'a> {
    pub texture: &'a dyn BlockTexture<'a>,
    pub rectangle: Rectangle<i32>,
    pub mapping: Rectangle<f32>,
}

pub trait BlockTexture<'a> {
    fn append_image(&self, image_data: &dyn ImageData) -> Block<'a>;
    fn clear(&self);
}
