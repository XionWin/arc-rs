use std::fmt::Debug;

use crate::{Command, Rect, Style};

pub trait Shape: Debug {
    fn get_commands(&self) -> &[Command];
    fn get_style(&self) -> &Style;
    fn get_rect(&self) -> Rect<i32>;
}
