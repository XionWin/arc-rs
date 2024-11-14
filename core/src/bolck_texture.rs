use crate::{ColorType, ImageData, Rect, RectF, Size, TextureFilter};

pub struct Block<'a> {
    pub texture: &'a dyn BlockTexture<'a>,
    pub rect: Rect,
    pub mapping: RectF
}

pub trait BlockTexture<'a> {
    fn get_id(&self) -> i32;
    fn get_size(&self) -> Size;
    fn get_color_type(&self) -> ColorType;
    fn get_texture_filter(&self) -> TextureFilter;
    
    fn append_image(&self, image_data: ImageData) -> Block<'a>;
    fn clear(&self);
}