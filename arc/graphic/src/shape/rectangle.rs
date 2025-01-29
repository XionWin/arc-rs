use core::{Command, Point, Shape, Size, Style};

#[derive(Debug)]
pub struct Rectangle {
    rect: core::Rectangle<i32>,
    style: Style,
    commands: Vec<Command>,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32, style: Style) -> Self {
        let commands = vec![
            Command::MoveTo(Point::new(x as _, y as _)),
            Command::LineTo(Point::new(x as _, (y + height) as _)),
            Command::LineTo(Point::new((x + width) as _, (y + height) as _)),
            Command::LineTo(Point::new((x + width) as _, y as _)),
            Command::Close,
        ];

        Self {
            rect: core::Rectangle::new(x, y, width, height),
            style,
            commands,
        }
    }
}

impl Shape for Rectangle {
    fn get_commands(&self) -> &[core::Command] {
        &self.commands
    }
    fn get_style(&self) -> &Style {
        &self.style
    }
    fn get_rect(&self) -> core::Rectangle<i32> {
        self.rect
    }
    fn get_size(&self) -> Size<i32> {
        self.rect.get_size()
    }
}
