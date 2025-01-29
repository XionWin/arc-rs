use std::{borrow::Borrow, rc::Rc};

use crate::Cache;

use super::Container;

pub struct Panel {
    _shapes: Vec<Rc<dyn core::Shape>>,
    _cache: Option<crate::TextureCache>,
    _rect: Option<core::Rect<i32>>,
}

impl Container for Panel {
    fn add(&mut self, shape: Box<dyn core::Shape>) {
        self.update_rect(shape.get_rect());
        self._shapes.push(shape.into());
    }
    fn get_children(&self) -> Vec<&dyn core::Shape> {
        self._shapes
            .iter()
            .map(|x| Borrow::<dyn core::Shape>::borrow(x))
            .collect::<Vec<&dyn core::Shape>>()
    }
}

impl Cache for Panel {
    fn get_rect(&self) -> Option<core::Rect<i32>> {
        self._rect
    }
    fn get_cache_texture(&self) -> Option<&crate::TextureCache> {
        match &self._cache {
            Some(cache) => Some(&cache),
            None => None,
        }
    }
}

impl Panel {
    pub fn new() -> Self {
        Self {
            _shapes: Vec::new(),
            _cache: None,
            _rect: None,
        }
    }
    fn update_rect(&mut self, rect: core::Rect<i32>) {
        match &mut self._rect {
            Some(mut self_rect) => self_rect += rect,
            None => self._rect = Some(rect),
        }
    }
}
