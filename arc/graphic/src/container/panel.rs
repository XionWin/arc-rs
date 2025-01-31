use core::AsAny;

use vector::VectorShape;

use crate::{Cacheable, Container, Element, PositionContainer};

#[derive(Debug)]
pub struct Panel {
    _position_container: PositionContainer,
    _cache: Option<crate::TextureCache>,
}

impl AsAny for Panel {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Element for Panel {}

impl Cacheable for Panel {
    fn get_rect(&self) -> Option<core::Rect<i32>> {
        self._position_container.get_rect()
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

impl Panel {
    pub fn new() -> Self {
        Self {
            _position_container: PositionContainer::new(),
            _cache: None,
        }
    }
}
