use std::rc::Rc;

use crate::TextureCache;

pub trait Container {
    fn add(drawable: Box<dyn core::Shape>);
    fn get_rect(&self) -> core::Rect<i32>;
    fn get_cache(&self) -> Rc<TextureCache>;
    fn get_children(&self) -> Vec<&dyn core::Shape>;
}
