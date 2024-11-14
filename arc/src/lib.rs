// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

mod location;
mod rect;
mod size;
mod skyline;
mod vertex;
mod window;
mod color;
mod graphic;
mod texture;

pub use location::*;
pub use rect::*;
pub use size::*;
pub use skyline::*;
pub use vertex::*;
pub use window::*;
pub use color::*;
pub use graphic::*;
pub use texture::*;
