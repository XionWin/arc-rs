use std::fmt::Debug;

use crate::{Command, Rectangle, Size, Style};

pub trait Shape: Debug {
    fn get_commands(&self) -> &[Command];
    fn get_style(&self) -> &Style;
    fn get_rect(&self) -> Rectangle<i32>;
    fn get_size(&self) -> Size<i32>;
}
