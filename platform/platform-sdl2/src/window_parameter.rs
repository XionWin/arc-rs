#[derive(Debug, Clone, Copy)]
pub enum VideoProfile {
    Core,
    GLES,
}

impl Into<sdl2::video::GLProfile> for VideoProfile {
    fn into(self) -> sdl2::video::GLProfile {
        match self {
            VideoProfile::Core => sdl2::video::GLProfile::Core,
            VideoProfile::GLES => sdl2::video::GLProfile::GLES,
        }
    }
}

impl std::fmt::Display for VideoProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct WindowParameter {
    pub profile: VideoProfile,
    pub version: core::Version
}

impl WindowParameter {
    pub fn new(profile:VideoProfile, version: core::Version) -> Self {
        Self {
            profile,
            version
        }
    }
}