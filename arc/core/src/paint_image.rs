use std::{borrow::Borrow, rc::Rc};

use crate::Matrix2D;

#[derive(Debug)]
pub struct PaintImage {
    _image: Rc<dyn crate::Image>,
    _transform: Matrix2D,
    _extent: crate::Extent<i32>,
}

impl PaintImage {
    pub fn new(image: Rc<dyn crate::Image>, view_port: crate::Rect<i32>) -> Self {
        Self {
            _image: image,
            _transform: Matrix2D::new_from_translate(
                -view_port.get_x() as _,
                -view_port.get_y() as _,
            ),
            _extent: crate::Extent::new(view_port.get_width(), view_port.get_height()),
        }
    }

    pub fn get_image(&self) -> &dyn crate::Image {
        self._image.borrow()
    }

    pub fn get_transform(&self) -> &Matrix2D {
        &self._transform
    }

    pub fn get_extent(&self) -> crate::Extent<i32> {
        self._extent
    }
}

impl From<Rc<dyn crate::Image>> for PaintImage {
    fn from(value: Rc<dyn crate::Image>) -> Self {
        let size = value.get_size();
        Self::new(
            value,
            crate::Rect::new(0i32, 0i32, size.get_width(), size.get_height()),
        )
    }
}
