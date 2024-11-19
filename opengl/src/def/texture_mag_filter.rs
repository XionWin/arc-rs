#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum TextureMagFilter
{
    Nearest = 9728,
    Linear = 9729,
    LinearDetailSgis = 32919,
    LinearDetailAlphaSgis = 32920,
    LinearDetailColorSgis = 32921,
    LinearSharpenSgis = 32941,
    LinearSharpenAlphaSgis = 32942,
    LinearSharpenColorSgis = 32943,
    Filter4Sgis = 33094,
    PixelTexGenQCeilingSgix = 33156,
    PixelTexGenQRoundSgix = 33157,
    PixelTexGenQFloorSgix = 33158
}

impl From<arc::TextureFilter> for TextureMagFilter {
    fn from(value: arc::TextureFilter) -> Self {
        match value {
            arc::TextureFilter::Nearest => TextureMagFilter::Nearest,
            arc::TextureFilter::Linear => TextureMagFilter::Linear,
        }
    }
}

