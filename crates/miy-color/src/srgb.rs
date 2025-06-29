use crate::rgb;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Channel {
    value: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Srgba {
    pub r: Channel,
    pub g: Channel,
    pub b: Channel,
    pub a: Channel,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Srgb {
    pub r: Channel,
    pub g: Channel,
    pub b: Channel,
}

impl Channel {
    pub const ZERO: Self = Self::new(0.0).unwrap();
    pub const ONE: Self = Self::new(1.0).unwrap();

    pub const fn new(value: f32) -> Option<Self> {
        if value < 0.0 || value > 1.0 {
            return None;
        }

        Some(Self { value })
    }

    pub const fn get(&self) -> f32 {
        self.value
    }
}

impl Srgb {
    pub const fn new(r: f32, g: f32, b: f32) -> Option<Self> {
        let Some(r) = Channel::new(r) else {
            return None;
        };
        let Some(g) = Channel::new(g) else {
            return None;
        };
        let Some(b) = Channel::new(b) else {
            return None;
        };

        Some(Self { r, g, b })
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.r.get(), self.g.get(), self.b.get()]
    }
}

impl Srgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Option<Self> {
        let Some(r) = Channel::new(r) else {
            return None;
        };
        let Some(g) = Channel::new(g) else {
            return None;
        };
        let Some(b) = Channel::new(b) else {
            return None;
        };
        let Some(a) = Channel::new(a) else {
            return None;
        };

        Some(Self { r, g, b, a })
    }

    pub fn to_array(self) -> [f32; 4] {
        [self.r.get(), self.g.get(), self.b.get(), self.a.get()]
    }
}

impl From<Srgb> for Srgba {
    fn from(Srgb { r, g, b }: Srgb) -> Self {
        let a = Channel::ONE;
        Self { r, g, b, a }
    }
}

impl From<Srgba> for Srgb {
    fn from(Srgba { r, g, b, .. }: Srgba) -> Self {
        Self { r, g, b }
    }
}

impl From<crate::Rgb32> for Srgb {
    fn from(crate::Rgb32 { r, g, b }: crate::Rgb32) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }
}

impl From<crate::Rgba32> for Srgba {
    fn from(crate::Rgba32 { r, g, b, a }: crate::Rgba32) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: Channel { value: a.get() },
        }
    }
}

impl From<crate::Rgb32> for Srgba {
    fn from(crate::Rgb32 { r, g, b }: crate::Rgb32) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: Channel::ONE,
        }
    }
}

impl From<rgb::Channel> for Channel {
    fn from(value: rgb::Channel) -> Self {
        let value = value.get();

        if value <= 0.0031308 {
            Self {
                value: 12.92 * value,
            }
        } else {
            Self {
                value: 1.055 * value.powf(1.0 / 2.4) - 0.055,
            }
        }
    }
}
