pub struct Version {
    pub major: u16,
    pub minor: u16
}

impl Version {
    pub fn new(major: impl Into<u16>, minor: impl Into<u16>) -> Self {
        Self {
            major: major.into(),
            minor: minor.into()
        }
    }
}