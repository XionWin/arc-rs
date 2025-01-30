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

impl Into<[f32; 4]> for &Rgba {
    fn into(self) -> [f32; 4] {
        [
            self.r as f32 / 255f32,
            self.g as f32 / 255f32,
            self.b as f32 / 255f32,
            self.a as f32 / 255f32,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Color {
    MidnightBlue,
    MoselleGreen,
    MagicDeepGray,
    White,
    Transparent,
    TrueColor { r: u8, g: u8, b: u8, a: u8 },
    HexColor { code: u32 },
}

impl Default for Color {
    fn default() -> Self {
        Self::Transparent
    }
}

impl Into<Rgba> for &Color {
    fn into(self) -> Rgba {
        match self {
            Color::MidnightBlue => Rgba {
                r: 0x19,
                g: 0x19,
                b: 0x70,
                a: 0xFF,
            },
            Color::MoselleGreen => Rgba {
                r: 0x2C,
                g: 0x4F,
                b: 0x3A,
                a: 0xFF,
            },
            Color::MagicDeepGray => Rgba {
                r: 0x1F,
                g: 0x20,
                b: 0x29,
                a: 0xFF,
            },
            Color::White => Rgba {
                r: 0xFF,
                g: 0xFF,
                b: 0xFF,
                a: 0xFF,
            },
            Color::Transparent => Rgba {
                r: 0x00,
                g: 0x00,
                b: 0x00,
                a: 0x00,
            },
            Color::TrueColor { r, g, b, a } => Rgba {
                r: *r,
                g: *g,
                b: *b,
                a: *a,
            },
            Color::HexColor { code } => Rgba {
                r: (code >> 8 * 3) as u8,
                g: (code >> 8 * 2) as u8,
                b: (code >> 8 * 1) as u8,
                a: (code >> 8 * 0) as u8,
            },
        }
    }
}

impl Into<Rgba> for Color {
    fn into(self) -> Rgba {
        Into::<Rgba>::into(&self)
    }
}

impl Into<[f32; 4]> for &Color {
    fn into(self) -> [f32; 4] {
        let rgba = Into::<Rgba>::into(self);
        Into::<[f32; 4]>::into(&rgba)
    }
}
