use crate::{ColorType, Size};

pub struct ImageData {
    pub size: Size<i32>,
    pub value: Vec<u8>,
    pub color_type: ColorType,
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
}

impl Drop for ImageData {
    fn drop(&mut self) {
        util::print_debug!("image_data {:?} droped", self.size);
    }
}