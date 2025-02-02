use core::AsAny;
use std::{borrow::Borrow, rc::Rc};

use vector::VectorShape;

use super::CacheableContainer;

#[derive(Debug)]
pub struct Container {
    _shapes: Vec<Rc<dyn core::Shape>>,
    _cache: crate::TextureCache,
}

impl Container {
    pub fn new(rectangle: core::Rectangle<i32>, texture: Rc<dyn crate::Texture>) -> Self {
        Self {
            _shapes: Vec::new(),
            _cache: crate::TextureCache::new(rectangle, core::Margin::default(), texture),
        }
    }

    fn update_cache(&mut self, cache: crate::TextureCache) {
        self._cache = cache;
    }
    fn get_cache(&self) -> &crate::TextureCache {
        &self._cache
    }
}

impl AsAny for Container {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn box_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl VectorShape for Container {
    fn get_stroke_primitive(&self) -> Option<vector::Primitive> {
        None
    }

    fn get_fill_primitive(&self) -> Option<vector::Primitive> {
        None
    }
}

impl core::Container for Container {
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

impl<T: core::Container + ?Sized> CacheableContainer for T {
    fn update_cache(&mut self, cache: crate::TextureCache) {
        match self.as_mut_any().downcast_mut::<Container>() {
            Some(container) => container.update_cache(cache),
            None => util::print_panic!("get texture from _texture_image failed"),
        }
    }

    fn get_cache(&self) -> &crate::TextureCache {
        match self.as_any().downcast_ref::<Container>() {
            Some(container) => container.get_cache(),
            None => util::print_panic!("get texture from _texture_image failed"),
        }
    }
}
