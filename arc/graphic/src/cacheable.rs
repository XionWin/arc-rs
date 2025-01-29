use crate::TextureCache;

pub trait Cacheable {
    fn get_rect(&self) -> Option<core::Rect<i32>>;
    fn get_cache(&self) -> Option<&TextureCache>;
}
