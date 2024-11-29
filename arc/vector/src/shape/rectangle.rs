use core::{Rect, Shape, Style};

#[derive(Debug)]
pub struct Rectangle {
    rect: Rect<i32>,
    style: Style,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            style: Style::default(),
        }
    }
}

impl Rectangle {
    pub fn get_rect(&self) -> &Rect<i32> {
        &self.rect
    }
    pub fn get_style(&self) -> &Style {
        &self.style
    }
}

impl Shape for Rectangle {
    fn get_commands(&self) -> &[core::Command] {
        todo!()
    }

    fn get_style(&self) -> &Style {
        todo!()
    }
}
