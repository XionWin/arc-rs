use std::rc::Rc;

use crate::TextureCache;

pub trait Cache {
    fn get_rect(&self) -> Option<core::Rect<i32>>;
    fn get_cache_texture(&self) -> Option<Rc<TextureCache>>;
}
