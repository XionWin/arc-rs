#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
    TrueColor { raw: Rgba },
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
            Color::TrueColor { raw } => raw,
        }
    }
}
