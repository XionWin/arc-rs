use crate::Texture;

pub trait TextureComponent: core::Image {
    fn get_texture(&self) -> &dyn Texture;
    fn get_texture_mut(&mut self) -> &mut dyn Texture;
}