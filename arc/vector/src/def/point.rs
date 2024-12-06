use std::fmt::Display;

use crate::def::PointFlag;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub flag: PointFlag,
    pub dx: Option<f32>,
    pub dy: Option<f32>,
    pub len: Option<f32>,
    pub dmx: Option<f32>,
    pub dmy: Option<f32>,
    pub dmr2: Option<f32>,
}

impl Point {
    pub fn new(x: f32, y: f32, flag: PointFlag) -> Self {
        Self {
            x,
            y,
            flag,
            dx: None,
            dy: None,
            len: None,
            dmx: None,
            dmy: None,
            dmr2: None,
        }
    }
    pub fn new_from_command_point(point: &crate::CommandPoint<f32>, flag: PointFlag) -> Self {
        Self {
            x: point.x,
            y: point.y,
            flag,
            dx: None,
            dy: None,
            len: None,
            dmx: None,
            dmy: None,
            dmr2: None,
        }
    }
    pub fn new_from_point(point: &Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
            flag: point.flag,
            dx: point.dx,
            dy: point.dy,
            len: point.len,
            dmx: point.dmx,
            dmy: point.dmy,
            dmr2: point.dmr2,
        }
    }

    pub fn get_point(&self) -> core::Point<f32> {
        core::Point::new(self.x, self.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{x:{:#.2}, y:{:#.2}, dx:{:#.2}, dy:{:#.2}, len:{:#.2} dmx:{:#.2}, dmy:{:#.2}, dmr2:{:#.2}, {:?}}}",
            self.x,
            self.y,
            self.dx.unwrap_or_default(),
            self.dy.unwrap_or_default(),
            self.len.unwrap_or_default(),
            self.dmx.unwrap_or_default(),
            self.dmy.unwrap_or_default(),
            self.dmr2.unwrap_or_default(),
            self.flag
        )
    }
}
