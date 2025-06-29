pub mod palette;
pub mod rgb;
pub mod srgb;

pub use rgb::{Rgb32, Rgba32};
pub use srgb::{Srgb, Srgba};

#[derive(Copy, Clone)]
pub enum Color {
    Rgba32(Rgba32),
    Rgb32(Rgb32),
    Srgba(Srgba),
    Srgb(Srgb),
}

impl Color {
    pub const BLACK: Self = Self::Rgb32(Rgb32::new(0.0, 0.0, 0.0).unwrap());
    pub const WHITE: Self = Self::Rgb32(Rgb32::new(1.0, 1.0, 1.0).unwrap());
    pub const RED: Self = Self::Rgb32(Rgb32::new(1.0, 0.0, 0.0).unwrap());
    pub const GREEN: Self = Self::Rgb32(Rgb32::new(0.0, 1.0, 0.0).unwrap());
    pub const BLUE: Self = Self::Rgb32(Rgb32::new(0.0, 0.0, 1.0).unwrap());

    // /// Almost black with a touch of green
    pub const DARK_JUNGLE_GREEN: Self = from_srgb(0.102, 0.141, 0.129);

    // /// Grape like purplee
    pub const PERSIAN_INDIGO: Self = from_srgb(0.20, 0.07, 0.47);

    // /// Dirty Whitee
    pub const GAINSBORO: Self = from_srgb(0.79, 0.92, 0.87);

    // /// It's really nice to look at
    pub const UNITY_YELLOW: Self = Self::Rgb32(Rgb32::new(1.0, 0.92, 0.016).unwrap());

    pub fn to_rgba32(self) -> Rgba32 {
        match self {
            Self::Rgba32(rgba) => rgba,
            Self::Rgb32(rgb) => rgb.into(),
            Self::Srgba(srgba) => srgba.into(),
            Self::Srgb(srgb) => srgb.into(),
        }
    }

    pub fn to_srgba(self) -> Srgba {
        match self {
            Color::Rgba32(rgba32) => rgba32.into(),
            Color::Rgb32(rgb32) => rgb32.into(),
            Color::Srgba(srgba) => srgba,
            Color::Srgb(srgb) => srgb.into(),
        }
    }

    pub fn to_rgb32(self) -> Rgb32 {
        match self {
            Self::Rgba32(rgba) => rgba.into(),
            Self::Rgb32(rgb) => rgb,
            Self::Srgba(srgba) => srgba.into(),
            Self::Srgb(srgb) => srgb.into(),
        }
    }
}

impl From<Rgb32> for Color {
    fn from(value: Rgb32) -> Self {
        Self::Rgb32(value)
    }
}

impl From<Rgba32> for Color {
    fn from(value: Rgba32) -> Self {
        Self::Rgba32(value)
    }
}

impl From<Srgb> for Color {
    fn from(value: Srgb) -> Self {
        Self::Srgb(value)
    }
}

impl From<Srgba> for Color {
    fn from(value: Srgba) -> Self {
        Self::Srgba(value)
    }
}

const fn from_srgb(r: f32, g: f32, b: f32) -> Color {
    Color::Srgb(Srgb::new(r, g, b).unwrap())
}
