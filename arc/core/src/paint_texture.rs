use std::{borrow::Borrow, rc::Rc};

use crate::Matrix2D;

#[derive(Debug)]
pub struct PaintTexture {
    _texture: Rc<dyn crate::Texture>,
    _transform: Matrix2D,
    _extent: crate::Extent<i32>,
}

impl PaintTexture {
    pub fn new(image: Rc<dyn crate::Texture>, view_port: crate::Rectangle<i32>) -> Self {
        Self {
            _texture: image,
            _transform: Matrix2D::new_from_translate(
                -view_port.get_x() as _,
                -view_port.get_y() as _,
            ),
            _extent: crate::Extent::new(view_port.get_width(), view_port.get_height()),
        }
    }

    pub fn get_texture(&self) -> &dyn crate::Texture {
        self._texture.borrow()
    }

    pub fn get_transform(&self) -> &Matrix2D {
        &self._transform
    }

    pub fn get_extent(&self) -> crate::Extent<i32> {
        self._extent
    }
}

impl From<Rc<dyn crate::Texture>> for PaintTexture {
    fn from(value: Rc<dyn crate::Texture>) -> Self {
        let size = value.get_size();
        Self::new(
            value,
            crate::Rectangle::new(0i32, 0i32, size.get_width(), size.get_height()),
        )
    }
}
