use core::{Command, Point, Shape, Size, Style};

use crate::{Cacheable, Drawable};

#[derive(Debug)]
pub struct Rectangle {
    rectangle: core::Rectangle<i32>,
    style: Style,
    commands: Vec<Command>,
}

impl Cacheable for Rectangle {
    fn get_rect(&self) -> Option<core::Rect<i32>> {
        todo!()
    }

    fn get_cache(&self) -> Option<&crate::TextureCache> {
        todo!()
    }
}

impl Drawable for Rectangle {
    fn get_container(&self) -> Option<&dyn crate::Container> {
        None
    }
}

impl Shape for Rectangle {
    fn get_commands(&self) -> &[core::Command] {
        &self.commands
    }
    fn get_style(&self) -> &Style {
        &self.style
    }
    fn get_rectangle(&self) -> core::Rectangle<i32> {
        self.rectangle
    }
    fn get_size(&self) -> Size<i32> {
        self.rectangle.get_size()
    }
    fn get_rect(&self) -> core::Rect<i32> {
        self.rectangle.into()
    }
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
            rectangle: core::Rectangle::new(x, y, width, height),
            style,
            commands,
        }
    }
}
