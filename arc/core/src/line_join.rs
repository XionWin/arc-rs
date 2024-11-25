pub enum LineJoin {
    Bevel,
    Miter,
    Round,
}

impl Default for LineJoin {
    fn default() -> Self {
        Self::Bevel
    }
}
