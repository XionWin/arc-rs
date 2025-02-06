#[derive(Debug, Clone, Copy)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

impl Default for TextureFilter {
    fn default() -> Self {
        Self::Linear
    }
}
