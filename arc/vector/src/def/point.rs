use crate::def::PointFlag;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub flags: PointFlag,
    _previous: Option<Box<Point>>,
    _next: Option<Box<Point>>,
    _len: Option<f32>,
    _dx: Option<f32>,
    _dy: Option<f32>,
    _dmx: Option<f32>,
    _dmy: Option<f32>,
    _dmr2: Option<f32>,
}

impl Point {
    pub fn new(x: f32, y: f32, flags: PointFlag) -> Self {
        Self {
            x,
            y,
            flags,
            _previous: None,
            _next: None,
            _len: None,
            _dx: None,
            _dy: None,
            _dmx: None,
            _dmy: None,
            _dmr2: None,
        }
    }
}
