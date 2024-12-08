use crate::TextureComponent;

pub trait TextureImage {
    fn try_get_texture(&self) -> Option<&dyn crate::Texture>;
}

impl<T: core::Image + ?Sized> TextureImage for T {
    fn try_get_texture(&self) -> Option<&dyn crate::Texture> {
        match self.as_any().downcast_ref::<crate::Image>() {
            Some(image) => Some(image.get_texture()),
            None => None,
        }
    }
}
