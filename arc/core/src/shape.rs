use std::fmt::Debug;

use crate::{Command, Style};

pub trait Shape: Debug {
    fn get_commands(&self) -> &[Command];
    fn get_style(&self) -> &Style;
}
