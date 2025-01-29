use std::{borrow::Borrow, rc::Rc};

use vector::VectorShape;

use crate::{Cacheable, Container, Drawable};

#[derive(Debug)]
pub struct Panel {
    _shapes: Vec<Rc<dyn core::Shape>>,
    _cache: Option<crate::TextureCache>,
    _rect: Option<core::Rect<i32>>,
}

impl Cacheable for Panel {
    fn get_rect(&self) -> Option<core::Rect<i32>> {
        self._rect
    }
    fn get_cache(&self) -> Option<&crate::TextureCache> {
        self._cache.as_ref()
    }
}

impl VectorShape for Panel {
    fn get_stroke_primitive(&self) -> Option<vector::Primitive> {
        None
    }

    fn get_fill_primitive(&self) -> Option<vector::Primitive> {
        None
    }
}

impl Drawable for Panel {
    fn get_container(&self) -> Option<&dyn Container> {
        None
    }
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
