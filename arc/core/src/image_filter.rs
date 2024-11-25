#[derive(Debug, Clone, Copy)]
pub enum ImageFilter {
    Linear,
    Nearest,
}

impl Default for ImageFilter {
    fn default() -> Self {
        Self::Linear
    }
}
