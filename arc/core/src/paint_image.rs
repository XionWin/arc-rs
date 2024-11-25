use std::rc::Rc;

pub struct PaintImage {
    _view_port: crate::Rect<i32>,
    _image: Option<Rc<dyn crate::Image>>,
    _extent: crate::Extent<i32>,
}

impl PaintImage {
    pub fn new(image: Rc<dyn crate::Image>, view_port: crate::Rect<i32>) -> Self {
        Self {
            _view_port: view_port,
            _image: Some(image),
            _extent: crate::Extent::new(view_port.size.width, view_port.size.height),
        }
    }
}
