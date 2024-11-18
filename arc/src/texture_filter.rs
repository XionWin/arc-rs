#[derive(Debug, Clone, Copy)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

impl From<core::ImageFilter> for TextureFilter {
    fn from(value: core::ImageFilter) -> Self {
        match value {
            core::ImageFilter::Linear => Self::Linear,
            core::ImageFilter::Nearest => Self::Nearest,
        }
    }
}

impl Into<core::ImageFilter> for TextureFilter {
    fn into(self) -> core::ImageFilter {
        match self {
            TextureFilter::Linear => core::ImageFilter::Linear,
            TextureFilter::Nearest => core::ImageFilter::Nearest,
        }
    }
}