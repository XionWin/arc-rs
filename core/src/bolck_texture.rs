use crate::{ImageData, Rect, RectF};

pub struct Block<'a> {
    pub texture: &'a dyn BlockTexture<'a>,
    pub rect: Rect,
    pub mapping: RectF
}

pub trait BlockTexture<'a> {
    fn append_image(&self, image_data: ImageData) -> Block<'a>;
    fn clear(&self);
}