use crate::{ImageData, Rect};

pub struct Block<'a> {
    pub texture: &'a dyn BlockTexture<'a>,
    pub rect: Rect<i32>,
    pub mapping: Rect<f32>
}

pub trait BlockTexture<'a> {
    fn append_image(&self, image_data: &dyn ImageData) -> Block<'a>;
    fn clear(&self);
}