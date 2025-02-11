#[cfg(test)]
pub mod test;

pub mod def;

mod parameter;
mod shape;
mod state;

pub use shape::*;
pub use state::*;

pub fn get_stroke_multiple(stroke_width: f32) -> f32 {
    use parameter::FRINGE_WIDTH;
    (stroke_width / 2f32 + FRINGE_WIDTH / 2f32) / FRINGE_WIDTH
}
