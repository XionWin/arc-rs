use crate::Texture;

pub struct Image<'a> {
    texture: Texture<'a>,
}

impl<'a> Image<'a> {
    pub fn new(texture: Texture<'a>) -> Self {
        Self {
            texture
        }
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn get_texture_mut(&mut self) -> &'a mut Texture {
        &mut self.texture
    }
}