mod version;
mod color_type;
mod texture_filter;
mod size;
mod image_data;

pub use version::*;
pub use color_type::*;
pub use texture_filter::*;
pub use size::*;
pub use image_data::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
