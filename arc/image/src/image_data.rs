use std::ffi::c_uchar;

use core::{ColorType, Size};

pub struct ImageData {
    size: Size<i32>,
    value: Vec<c_uchar>,
    color_type: ColorType,
}

impl ImageData {
    pub fn new(size: Size<i32>, value: impl Into<Vec<u8>>, color_type: ColorType) -> Self {
        Self {
            size,
            value: value.into(),
            color_type,
        }
    }

    pub fn new_from_file(path: &str) -> Self {
        let image = image::ImageReader::open(path).unwrap().decode().unwrap();
        let size = Size::new(image.width() as _, image.height() as _);
        Self {
            size,
            value: image.to_rgba8().into_vec(),
            color_type: ColorType::Rgba,
        }
    }

    pub fn export_to_file(buffer: &[u8], size: core::Size<i32>, path: &str) {
        image::save_buffer(
            path,
            buffer,
            size.get_width() as _,
            size.get_height() as _,
            image::ExtendedColorType::Rgba8,
        )
        .unwrap()
    }
}

impl core::ImageData for ImageData {
    fn get_size(&self) -> core::Size<i32> {
        self.size
    }

    fn get_color_type(&self) -> core::ColorType {
        self.color_type
    }

    fn get_value(&self) -> &Vec<c_uchar> {
        &self.value
    }
}

impl Drop for ImageData {
    fn drop(&mut self) {
        util::print_debug!("image_data {:?} droped", self.size);
    }
}
