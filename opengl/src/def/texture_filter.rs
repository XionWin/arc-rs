#[derive(Debug, Clone, Copy)]
pub enum TextureFilter {
    Nearest,
    Linear,
}

impl Into<TextureMinFilter> for TextureFilter {
    fn into(self) -> TextureMinFilter {
        match self {
            Self::Nearest => TextureMinFilter::Nearest,
            Self::Linear => TextureMinFilter::Linear,
        }
    }
}

impl Into<TextureMagFilter> for TextureFilter {
    fn into(self) -> TextureMagFilter {
        match self {
            Self::Nearest => TextureMagFilter::Nearest,
            Self::Linear => TextureMagFilter::Linear,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum TextureMinFilter {
    Nearest = 9728,
    Linear = 9729,
    NearestMipmapNearest = 9984,
    LinearMipmapNearest = 9985,
    NearestMipmapLinear = 9986,
    LinearMipmapLinear = 9987,
    Filter4Sgis = 33094,
    LinearClipmapLinearSgix = 33136,
    PixelTexGenQCeilingSgix = 33156,
    PixelTexGenQRoundSgix = 33157,
    PixelTexGenQFloorSgix = 33158,
    NearestClipmapNearestSgix = 33869,
    NearestClipmapLinearSgix = 33870,
    LinearClipmapNearestSgix = 33871
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum TextureMagFilter {
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