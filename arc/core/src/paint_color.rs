#[derive(Debug)]
pub struct PaintColor {
    _inner_color: crate::Color,
    _outer_color: crate::Color,
}

impl PaintColor {
    pub fn new(inner_color: crate::Color, outer_color: crate::Color) -> Self {
        Self {
            _inner_color: inner_color,
            _outer_color: outer_color,
        }
    }

    pub fn get_inner_color(&self) -> crate::Color {
        self._inner_color
    }

    pub fn get_outer_color(&self) -> crate::Color {
        self._outer_color
    }
}

impl Default for PaintColor {
    fn default() -> Self {
        Self {
            _inner_color: crate::Color::default(),
            _outer_color: crate::Color::default(),
        }
    }
}
