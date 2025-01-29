use core::{Command, Point, Rectangle, Shape, Size, Style};

const KAPPA90: f32 = 0.5522847493f32;

#[derive(Debug)]
pub struct RoundRectangle {
    rect: Rectangle<i32>,
    style: Style,
    commands: Vec<Command>,
}

impl RoundRectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32, r: i32, style: Style) -> Self {
        let commands = {
            let x = x as f32;
            let y = y as f32;
            let width = width as f32;
            let height = height as f32;
            let r = r as f32;
            vec![
                Command::MoveTo(Point::new(x, y)),
                Command::LineTo(Point::new(x + width - r, y)),
                Command::BezierTo(
                    Point::new(x + width - r + r * KAPPA90, y),
                    Point::new(x + width, y + r - r * KAPPA90),
                    Point::new(x + width, y + r),
                ),
                Command::LineTo(Point::new(x + width, y + height - r)),
                Command::BezierTo(
                    Point::new(x + width, y + height - r + r * KAPPA90),
                    Point::new(x + width - r + r * KAPPA90, y + height),
                    Point::new(x + width - r, y + height),
                ),
                Command::LineTo(Point::new(x + r, y + height)),
                Command::BezierTo(
                    Point::new(x + r - r * KAPPA90, y + height),
                    Point::new(x, y + height - r + r * KAPPA90),
                    Point::new(x, y + height - r),
                ),
                Command::LineTo(Point::new(x, y + r)),
                Command::BezierTo(
                    Point::new(x, y + r - r * KAPPA90),
                    Point::new(x + r - r * KAPPA90, y),
                    Point::new(x + r, y),
                ),
                Command::Close,
            ]
        };

        Self {
            rect: Rectangle::new(x, y, width, height),
            style,
            commands,
        }
    }
}

impl Shape for RoundRectangle {
    fn get_commands(&self) -> &[core::Command] {
        &self.commands
    }
    fn get_style(&self) -> &Style {
        &self.style
    }
    fn get_rect(&self) -> Rectangle<i32> {
        self.rect
    }
    fn get_size(&self) -> Size<i32> {
        self.rect.get_size()
    }
}
