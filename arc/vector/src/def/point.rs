use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

use crate::def::PointFlag;

#[derive(Debug)]
pub struct Point {
    point: core::Point<f32>,
    pub flag: PointFlag,
    _previous: Option<Weak<RefCell<Point>>>,
    _next: Option<Rc<RefCell<Point>>>,
    _len: Option<f32>,
    _dx: Option<f32>,
    _dy: Option<f32>,
    _dmx: Option<f32>,
    _dmy: Option<f32>,
    _dmr2: Option<f32>,
}

impl Point {
    pub fn new(x: f32, y: f32, flag: PointFlag) -> Self {
        Self {
            point: core::Point::new(x, y),
            flag,
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
    pub fn new_from_point(point: &core::Point<f32>, flags: PointFlag) -> Self {
        Self {
            point: point.clone(),
            flag: flags,
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

    pub fn set_next(&mut self, next: Rc<RefCell<Point>>) {
        self._next = Some(next);
    }

    pub fn set_previous(&mut self, previous: Weak<RefCell<Point>>) {
        self._previous = Some(previous);
    }

    pub fn get_next(&self) -> Option<Rc<RefCell<Point>>> {
        match &self._next {
            Some(next) => Some(next.clone()),
            None => None,
        }
    }

    pub fn get_previous(&self) -> Option<Weak<RefCell<Point>>> {
        match &self._previous {
            Some(previous) => Some(previous.clone()),
            None => None,
        }
    }

    pub fn get_point_ref(&self) -> &core::Point<f32> {
        &self.point
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self._next {
            Some(next) => {
                let r: std::cell::Ref<'_, Point> = next.borrow();
                write!(
                    f,
                    "\n[{}, {}], {:?}, {}",
                    self.point.x, self.point.y, self.flag, r
                )
            }
            None => write!(f, "\n[{}, {}], {:?}", self.point.x, self.point.y, self.flag),
        }
    }
}

impl Iterator for Point {
    type Item = Rc<RefCell<Point>>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self._next {
            Some(next) => Some(next.clone()),
            None => None,
        }
    }
}
