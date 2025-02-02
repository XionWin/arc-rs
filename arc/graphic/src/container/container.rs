use core::AsAny;
use std::{borrow::Borrow, rc::Rc};

use vector::VectorShape;

use super::CacheableContainer;

#[derive(Debug)]
pub struct Container {
    _shapes: Vec<Rc<dyn core::Shape>>,
    _cache: crate::TextureCache,
}

impl VectorShape for Container {
    fn get_stroke_primitive(&self) -> Option<vector::Primitive> {
        None
    }

    fn get_fill_primitive(&self) -> Option<vector::Primitive> {
        None
    }
}

impl AsAny for Container {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CacheableContainer for Container {
    fn add(&mut self, shape: Box<dyn core::Shape>) {
        self._shapes.push(shape.into());
    }

    fn get_children(&self) -> Vec<&dyn core::Shape> {
        self._shapes
            .iter()
            .map(|x| Borrow::<dyn core::Shape>::borrow(x))
            .collect::<Vec<&dyn core::Shape>>()
    }

    fn update_cache(&mut self, cache: crate::TextureCache) {
        self._cache = cache;
    }

    fn get_cache(&self) -> &crate::TextureCache {
        &self._cache
    }
}

impl Container {
    pub fn new(rectangle: core::Rectangle<i32>, texture: Rc<dyn crate::Texture>) -> Self {
        Self {
            _shapes: Vec::new(),
            _cache: crate::TextureCache::new(rectangle, core::Margin::default(), texture),
        }
    }
}
