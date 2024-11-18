use core::Texture;

pub trait Renderer {
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn clear_color(&self, color: crate::Color);
    fn clear(&self);

    fn create_texture(
        &self,
        size: core::Size<i32>,
        color_type: core::ColorType,
        color_filter: core::TextureFilter,
    ) -> Box<dyn Texture + '_>;
    // fn create_texture_with_file(
    //     &self,
    //     path: &str,
    //     color_type: core::ColorType,
    //     color_filter: core::TextureFilter,
    // ) -> Texture;
    // fn create_texture_with_image_data(
    //     &self,
    //     image_data: core::ImageData,
    //     color_filter: core::TextureFilter,
    // ) -> Texture;
    fn drop_texture(&self, texture: &dyn core::Texture);
    fn drop_texture_by_id(&self, texture_id: i32);
}
