#[derive(Debug, Clone, Copy)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    /// update version (patchlevel)
    pub patch: u8,
}

impl Version {
    pub fn new(major: impl Into<u8>, minor: impl Into<u8>, patch: impl Into<u8>) -> Self {
        Self {
            major: major.into(),
            minor: minor.into(),
            patch: patch.into()
        }
    }
}