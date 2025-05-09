#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum TextureMinFilter {
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
    LinearClipmapNearestSgix = 33871,
}

impl From<core::TextureFilter> for TextureMinFilter {
    fn from(value: core::TextureFilter) -> Self {
        match value {
            core::TextureFilter::Nearest => TextureMinFilter::Nearest,
            core::TextureFilter::Linear => TextureMinFilter::Linear,
        }
    }
}
