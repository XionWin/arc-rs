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
            _transform: {
                let mat = Matrix2D::default();
                mat.translate(-view_port.location.x as _, -view_port.location.y as _);
                mat
            },
            _extent: crate::Extent::new(view_port.size.width, view_port.size.height),
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
        Self::new(value, crate::Rect::new(0i32, 0i32, size.width, size.height))
    }
}
