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
    pub fn new_with_inner_color(inner_color: crate::Color) -> Self {
        Self {
            _inner_color: inner_color,
            _outer_color: crate::Color::Transparent,
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

impl Clone for PaintColor {
    fn clone(&self) -> Self {
        Self {
            _inner_color: self._inner_color.clone(),
            _outer_color: self._outer_color.clone(),
        }
    }
}
