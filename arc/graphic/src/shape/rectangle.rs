use core::{Command, Point, Shape, Size, Style};

#[derive(Debug)]
pub struct Rectangle {
    rectangle: core::Rectangle<i32>,
    style: Style,
    commands: Vec<Command>,
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
    // pub fn new(x: i32, y: i32, width: i32, height: i32, style: Style) -> Self {
    //     let commands = vec![
    //         Command::MoveTo(Point::new(x as _, y as _)),
    //         Command::LineTo(Point::new(x as _, (y + height) as _)),
    //         Command::LineTo(Point::new((x + width) as _, (y + height) as _)),
    //         Command::LineTo(Point::new((x + width) as _, y as _)),
    //         Command::Close,
    //     ];

    //     Self {
    //         rectangle: core::Rectangle::new(x, y, width, height),
    //         style,
    //         commands,
    //     }
    // }
    pub fn new(x: i32, y: i32, width: i32, height: i32, style: Style) -> Self {
        let start_x = 0f32;
        let start_y: f32 = 0f32;
        let commands = {
            let width = width as f32;
            let height = height as f32;
            vec![
                Command::MoveTo(Point::new(start_x as _, start_y as _)),
                Command::LineTo(Point::new(start_x as _, (start_y + height) as _)),
                Command::LineTo(Point::new((start_x + width) as _, (start_y + height) as _)),
                Command::LineTo(Point::new((start_x + width) as _, start_y as _)),
                Command::Close,
            ]
        };

        Self {
            rectangle: core::Rectangle::new(x, y, width, height),
            style,
            commands,
        }
    }
}
