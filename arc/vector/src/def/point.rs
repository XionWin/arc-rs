use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

use crate::def::PointFlag;

#[derive(Debug)]
pub struct Point {
    pub point: core::Point<f32>,
    pub flag: PointFlag,
    _previous: Option<Weak<RefCell<Point>>>,
    _next: Option<Rc<RefCell<Point>>>,
    pub len: Option<f32>,
    pub dx: Option<f32>,
    pub dy: Option<f32>,
    pub dmx: Option<f32>,
    pub dmy: Option<f32>,
    pub dmr2: Option<f32>,
}

impl Point {
    pub fn new(x: f32, y: f32, flag: PointFlag) -> Self {
        Self {
            point: core::Point::new(x, y),
            flag,
            _previous: None,
            _next: None,
            len: None,
            dx: None,
            dy: None,
            dmx: None,
            dmy: None,
            dmr2: None,
        }
    }
    pub fn new_from_command_point(point: &crate::CommandPoint<f32>, flags: PointFlag) -> Self {
        Self {
            point: point.get_point(),
            flag: flags,
            _previous: None,
            _next: None,
            len: None,
            dx: None,
            dy: None,
            dmx: None,
            dmy: None,
            dmr2: None,
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
                    "{{x:{:#.2}, y:{:#.2}, dx:{:#.2}, dy:{:#.2}, len:{:#.2}, dmx:{:#.2}, dmy:{:#.2}, dmr2:{:#.2}, {:?}, has_previous?: {}}}\n{}",
                    self.point.x,
                    self.point.y,
                    self.dx.unwrap_or_default(),
                    self.dy.unwrap_or_default(),
                    self.len.unwrap_or_default(),
                    self.dmx.unwrap_or_default(),
                    self.dmy.unwrap_or_default(),
                    self.dmr2.unwrap_or_default(),
                    self.flag,
                    self._previous.is_some(),
                    r
                )
            }
            None => write!(
                f,
                "{{x:{:#.2}, y:{:#.2}, dx:{:#.2}, dy:{:#.2}, len:{:#.2}, dmx:{:#.2}, dmy:{:#.2}, dmr2:{:#.2}, {:?}, has_previous?: {}}}",
                self.point.x,
                self.point.y,
                self.dx.unwrap_or_default(),
                self.dy.unwrap_or_default(),
                self.len.unwrap_or_default(),
                self.dmx.unwrap_or_default(),
                self.dmy.unwrap_or_default(),
                self.dmr2.unwrap_or_default(),
                self.flag,
                self._previous.is_some()
            ),
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
