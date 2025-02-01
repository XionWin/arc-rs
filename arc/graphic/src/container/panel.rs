use vector::VectorShape;

use crate::PositionContainer;

#[derive(Debug)]
pub struct Panel {
    _position_container: PositionContainer,
    _cache: Option<crate::TextureCache>,
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
