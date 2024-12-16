#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Rgba,
    Alpha,
}

impl Default for ColorType {
    fn default() -> Self {
        Self::Rgba
    }
}
