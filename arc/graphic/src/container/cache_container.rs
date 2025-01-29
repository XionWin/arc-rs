use std::{borrow::Borrow, rc::Rc};

use crate::Cache;

use super::Container;

pub struct CacheContainer {
    _shapes: Vec<Rc<dyn core::Shape>>,
    _rect: Option<core::Rect<i32>>,
}

impl Container for CacheContainer {
    fn add(&mut self, shape: Box<dyn core::Shape>) {
        self._shapes.push(shape.into());
    }

    fn get_children(&self) -> Vec<&dyn core::Shape> {
        self._shapes
            .iter()
            .map(|x| Borrow::<dyn core::Shape>::borrow(x))
            .collect::<Vec<&dyn core::Shape>>()
    }
}

impl Cache for CacheContainer {
    fn get_rect(&self) -> Option<core::Rect<i32>> {
        todo!()
    }

    fn get_cache_texture(&self) -> Option<Rc<crate::TextureCache>> {
        todo!()
    }
}

impl CacheContainer {
    pub fn new() -> Self {
        Self {
            _shapes: Vec::new(),
            _rect: None,
        }
    }
}
