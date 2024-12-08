use std::rc::Rc;

pub trait TextureImage {
    fn get_texture(&self) -> Rc<dyn graphic::Texture>;
}
