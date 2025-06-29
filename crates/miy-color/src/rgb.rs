use crate::srgb;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Channel {
    value: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rgba32 {
    pub r: Channel,
    pub g: Channel,
    pub b: Channel,
    pub a: Channel,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rgb32 {
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

impl Rgb32 {
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

impl Rgba32 {
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

impl From<Rgb32> for Rgba32 {
    fn from(Rgb32 { r, g, b }: Rgb32) -> Self {
        Self {
            r,
            g,
            b,
            a: Channel::ONE,
        }
    }
}

impl From<Rgba32> for Rgb32 {
    fn from(Rgba32 { r, g, b, .. }: Rgba32) -> Self {
        Self { r, g, b }
    }
}

impl From<srgb::Srgb> for Rgb32 {
    fn from(srgb::Srgb { r, g, b }: srgb::Srgb) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }
}

impl From<srgb::Srgba> for Rgb32 {
    fn from(srgb::Srgba { r, g, b, .. }: srgb::Srgba) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }
}

impl From<srgb::Srgb> for Rgba32 {
    fn from(srgb::Srgb { r, g, b }: srgb::Srgb) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: Channel::ONE,
        }
    }
}
impl From<srgb::Srgba> for Rgba32 {
    fn from(srgb::Srgba { r, g, b, a }: srgb::Srgba) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: Channel { value: a.get() },
        }
    }
}

impl From<srgb::Channel> for Channel {
    fn from(value: srgb::Channel) -> Self {
        let value = value.get();
        if value <= 0.04045 {
            Self {
                value: value / 12.92,
            }
        } else {
            Self {
                value: ((value + 0.055) / 1.055).powf(2.4),
            }
        }
    }
}
