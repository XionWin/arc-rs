#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Into<(u8, u8, u8, u8)> for Rgba {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

impl Into<(f32, f32, f32, f32)> for Rgba {
    fn into(self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255f32,
            self.g as f32 / 255f32,
            self.b as f32 / 255f32,
            self.a as f32 / 255f32,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    MidnightBlue,
    Transparent,
    TrueColor { r: u8, g: u8, b: u8, a: u8 },
}

impl Default for Color {
    fn default() -> Self {
        Self::Transparent
    }
}

impl Into<Rgba> for Color {
    fn into(self) -> Rgba {
        match self {
            Color::Black => Rgba {
                r: 0u8,
                g: 0u8,
                b: 0u8,
                a: 255u8,
            },
            Color::Red => Rgba {
                r: 205u8,
                g: 0u8,
                b: 0u8,
                a: 255u8,
            },
            Color::Green => Rgba {
                r: 0u8,
                g: 205u8,
                b: 0u8,
                a: 255u8,
            },
            Color::Yellow => Rgba {
                r: 205u8,
                g: 205u8,
                b: 0u8,
                a: 255u8,
            },
            Color::Blue => Rgba {
                r: 0u8,
                g: 0u8,
                b: 238u8,
                a: 255u8,
            },
            Color::Magenta => Rgba {
                r: 205u8,
                g: 0u8,
                b: 205u8,
                a: 255u8,
            },
            Color::Cyan => Rgba {
                r: 0u8,
                g: 205u8,
                b: 205u8,
                a: 255u8,
            },
            Color::White => Rgba {
                r: 229u8,
                g: 229u8,
                b: 229u8,
                a: 255u8,
            },
            Color::BrightBlack => Rgba {
                r: 127u8,
                g: 127u8,
                b: 127u8,
                a: 255u8,
            },
            Color::BrightRed => Rgba {
                r: 255u8,
                g: 0u8,
                b: 0u8,
                a: 255u8,
            },
            Color::BrightGreen => Rgba {
                r: 0u8,
                g: 255u8,
                b: 0u8,
                a: 255u8,
            },
            Color::BrightYellow => Rgba {
                r: 255u8,
                g: 255u8,
                b: 0u8,
                a: 255u8,
            },
            Color::BrightBlue => Rgba {
                r: 92u8,
                g: 92u8,
                b: 255u8,
                a: 255u8,
            },
            Color::BrightMagenta => Rgba {
                r: 255u8,
                g: 0u8,
                b: 255u8,
                a: 255u8,
            },
            Color::BrightCyan => Rgba {
                r: 0u8,
                g: 255u8,
                b: 255u8,
                a: 255u8,
            },
            Color::BrightWhite => Rgba {
                r: 255u8,
                g: 255u8,
                b: 255u8,
                a: 255u8,
            },
            Color::MidnightBlue => Rgba {
                r: 25u8,
                g: 25u8,
                b: 112u8,
                a: 255u8,
            },
            Color::Transparent => Rgba {
                r: 0u8,
                g: 0u8,
                b: 0u8,
                a: 0u8,
            },
            Color::TrueColor { r, g, b, a } => Rgba { r, g, b, a },
        }
    }
}
