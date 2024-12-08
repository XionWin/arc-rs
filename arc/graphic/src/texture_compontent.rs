use crate::Texture;

pub trait TextureComponent: core::Image {
    fn get_texture(&self) -> &dyn Texture;
}
