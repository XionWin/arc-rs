#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    Rgba,
    Alpha,
}

impl Default for ColorType {
    fn default() -> Self {
        Self::Rgba
    }
}
