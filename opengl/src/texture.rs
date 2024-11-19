use std::rc::Rc;

pub struct Texture {
    renderer: Rc<dyn arc::Renderer>,
    id: u32,
    size: core::Size<i32>,
    color_type: core::ColorType,
    texture_filter: arc::TextureFilter
}

impl Texture {
    pub fn new(renderer: Rc<dyn arc::Renderer>, size: core::Size<i32>, color_type: core::ColorType, texture_filter: arc::TextureFilter) -> Self {
        Self {
            renderer,
            id: crate::gl::gen_texture(),
            size,
            color_type,
            texture_filter
        }
    }
}

impl arc::Texture for Texture {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_size(&self) -> core::Size<i32> {
        self.size
    }

    fn get_color_type(&self) -> core::ColorType {
        self.color_type
    }

    fn get_texture_filter(&self) -> arc::TextureFilter {
        self.texture_filter
    }

    fn load(&self, image_data: &core::ImageData) {
        crate::gl::bind_texture(crate::def::TextureTarget::Texture2D, self.id);

        crate::gl::tex_image_2d(crate::def::TextureTarget::Texture2D, 0, crate::def::PixelInternalFormat::Rgba, image_data.size.width, image_data.size.height, 0, crate::def::PixelFormat::Rgba, crate::def::PixelType::UnsignedByte, Some(&image_data.value));

        // Now that our texture is loaded, we can set a few settings to affect how the image appears on rendering.

        // First, we set the min and mag filter. These are used for when the texture is scaled down and up, respectively.
        // Here, we use Linear for both. This means that OpenGL will try to blend pixels, meaning that textures scaled too far will look blurred.
        // You could also use (amongst other options) Nearest, which just grabs the nearest pixel, which makes the texture look pixelated if scaled too far.
        // NOTE: The default settings for both of these are LinearMipmap. If you leave these as default but don't generate mipmaps,
        // your image will fail to render at all (usually resulting in pure black instead).
        crate::gl::tex_parameter_i(crate::def::TextureTarget::Texture2D, crate::def::TextureParameterName::TextureMinFilter, Into::<arc::TextureFilter>::into(self.texture_filter) as _);
        crate::gl::tex_parameter_i(crate::def::TextureTarget::Texture2D, crate::def::TextureParameterName::TextureMagFilter, Into::<arc::TextureFilter>::into(self.texture_filter) as _);

        // Now, set the wrapping mode. S is for the X axis, and T is for the Y axis.
        // We set this to Repeat so that textures will repeat when wrapped. Not demonstrated here since the texture coordinates exactly match
        crate::gl::tex_parameter_i(crate::def::TextureTarget::Texture2D, crate::def::TextureParameterName::TextureWrapS, crate::def::TextureWrapMode::Repeat as _);
        crate::gl::tex_parameter_i(crate::def::TextureTarget::Texture2D, crate::def::TextureParameterName::TextureWrapT, crate::def::TextureWrapMode::Repeat as _);

        // Next, generate mipmaps.
        // Mipmaps are smaller copies of the texture, scaled down. Each mipmap level is half the size of the previous one
        // Generated mipmaps go all the way down to just one pixel.
        // OpenGL will automatically switch between mipmaps when an object gets sufficiently far away.
        // This prevents moiré effects, as well as saving on texture bandwidth.
        // Here you can see and read about the morié effect https://en.wikipedia.org/wiki/Moir%C3%A9_pattern
        // Here is an example of mips in action https://en.wikipedia.org/wiki/File:Mipmap_Aliasing_Comparison.png
        crate::gl::generate_mipmap(crate::def::GenerateMipmapTarget::Texture2D);
    }

    fn export(&self, path: &str) {
        println!("path: {}", path);
        todo!()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.renderer.drop_texture(self);
        util::print_debug!("texture {} droped", self.id);
    }
}