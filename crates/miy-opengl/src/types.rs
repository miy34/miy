use core::{ffi, fmt, ops};

// basic OpenGL types
pub type Byte = ffi::c_char;
pub type Ubyte = ffi::c_uchar;
pub type Char = ffi::c_char;
pub type Bitfield = ffi::c_uint;
pub type Enum = ffi::c_uint;
pub type Fixed = Int;
pub type Float = ffi::c_float;
pub type Half = ffi::c_ushort;
pub type Int = ffi::c_int;
pub type Intptr = isize;
pub type Sizei = ffi::c_int;
pub type Sizeiptr = isize;
pub type Uint = ffi::c_uint;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bool(ffi::c_uchar);
impl Bool {
    pub const TRUE: Self = Self(1);
    pub const FALSE: Self = Self(0);

    pub fn from_int(value: Int) -> Option<Self> {
        match value {
            0 => Some(Self::FALSE),
            1 => Some(Self::TRUE),
            _ => None,
        }
    }
}

// Type safe wrappers over basic OpenGL types

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Error(Enum);
impl Error {
    pub const NO_ERROR: Self = Self(0x0);
    pub const INVALID_ENUM: Self = Self(0x0500);
    pub const INVALID_VALUE: Self = Self(0x0501);
    pub const INVALID_OPERATION: Self = Self(0x0502);
    pub const STACK_OVERFLOW: Self = Self(0x0503);
    pub const STACK_UNDERFLOW: Self = Self(0x0504);
    pub const OUT_OF_MEMORY: Self = Self(0x0505);
    pub const INVALID_FRAMEBUFFER_OPERATION: Self = Self(0x0506);
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::NO_ERROR => write!(f, "no error"),
            Self::INVALID_ENUM => write!(f, "invalid enum"),
            Self::INVALID_VALUE => write!(f, "invalid value"),
            Self::INVALID_OPERATION => write!(f, "invalid operation"),
            Self::STACK_OVERFLOW => write!(f, "stack overflow"),
            Self::STACK_UNDERFLOW => write!(f, "stack underflow"),
            Self::OUT_OF_MEMORY => write!(f, "out of memory"),
            Self::INVALID_FRAMEBUFFER_OPERATION => write!(f, "invalid framebuffer operation"),
            _ => write!(f, "unknown error"),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Description(Enum);
impl Description {
    pub const VENDOR: Self = Self(0x1F00);
    pub const RENDERER: Self = Self(0x1F01);
    pub const VERSION: Self = Self(0x1F02);
    pub const SHADING_LANGUAGE_VERSION: Self = Self(0x8B8C);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Capability(Enum);
impl Capability {
    pub const DEBUG_OUTPUT: Self = Self(0x92E0);
    pub const DEPTH: Self = Self(0x0B71);
    pub const CULL_FACE: Self = Self(0x0B44);
    pub const BLEND: Self = Self(0x0BE2);
    pub const MULTI_SAMPLE: Self = Self(0x809D);
    pub const FRAMEBUFFER_SRGB: Self = Self(0x8DB9);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ClearMask(Bitfield);
impl ClearMask {
    pub const NONE: Self = Self(0);
    pub const COLOR: Self = Self(0x4000);
    pub const DEPTH: Self = Self(0x100);
    pub const STENCIL: Self = Self(0x400);
    //TODO: replace with const impl of bitor once it stabilizes
    pub const ALL: Self = Self(Self::COLOR.0 | Self::DEPTH.0 | Self::STENCIL.0);

    #[must_use]
    pub const fn is_some(self) -> bool {
        self.0 != 0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Primitive(Enum);
impl Primitive {
    pub const TRIANGLES: Self = Self(0x4);
    pub const TRIANGLE_STRIP: Self = Self(0x0005);
    pub const TRIANGLE_FAN: Self = Self(0x0006);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Start(pub Int);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Count(pub Sizei);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ElementKind(Enum);
impl ElementKind {
    pub const UNSIGNED_BYTE: Self = Self(0x1401);
    pub const UNSIGNED_SHORT: Self = Self(0x1403);
    pub const UNSIGNED_INT: Self = Self(0x1405);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VertexArray(Uint);

impl VertexArray {
    pub const NONE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Buffer(pub Uint);

impl Buffer {
    pub const NONE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferTarget(Enum);
impl BufferTarget {
    pub const ARRAY_BUFFER: Self = Self(0x8892);
    pub const COPY_READ_BUFFER: Self = Self(0x8F36);
    pub const COPY_WRITE_BUFFER: Self = Self(0x8F37);
    pub const ELEMENT_ARRAY_BUFFER: Self = Self(0x8893);
    pub const UNIFORM_BUFFER: Self = Self(0x8A11);
    pub const TEXTURE_BUFFER: Self = Self(0x8C2A);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferUsage(Enum);
impl BufferUsage {
    pub const STREAM_DRAW: Self = Self(0x88E0);
    pub const STREAM_READ: Self = Self(0x88E1);
    pub const STREAM_COPY: Self = Self(0x88E2);
    pub const STATIC_DRAW: Self = Self(0x88E4);
    pub const STATIC_READ: Self = Self(0x88E5);
    pub const STATIC_COPY: Self = Self(0x88E6);
    pub const DYNAMIC_DRAW: Self = Self(0x88E8);
    pub const DYNAMIC_READ: Self = Self(0x88E9);
    pub const DYNAMIC_COPY: Self = Self(0x88EA);
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BufferBinding {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    ELEVEN,
    TWELVE,
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Shader(Uint);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ShaderKind(Enum);
impl ShaderKind {
    pub const FRAGMENT: Self = Self(0x8B30);
    pub const VERTEX: Self = Self(0x8B31);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Program(Uint);
impl Program {
    pub const NONE: Self = Self(0);

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ShaderParameterName(Enum);
impl ShaderParameterName {
    pub const COMPILE_STATUS: Self = Self(0x8B81);
    pub const INFO_LOG_LENGTH: Self = Self(0x8B84);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlendFactor(Enum);
impl BlendFactor {
    pub const ZERO: Self = Self(0x0000);
    pub const ONE: Self = Self(0x0001);
    pub const SRC_COLOR: Self = Self(0x0300);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(0x0301);
    pub const DST_COLOR: Self = Self(0x0306);
    pub const ONE_MINUS_DST_COLOR: Self = Self(0x0307);
    pub const SRC_ALPHA: Self = Self(0x0302);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(0x0303);
    pub const DST_ALPHA: Self = Self(0x0304);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(0x0305);
    pub const CONSTANT_COLOR: Self = Self(0x8001);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(0x8002);
    pub const CONSTANT_ALPHA: Self = Self(0x8003);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(0x8004);
    pub const SRC_ALPHA_SATURATE: Self = Self(0x0308);
    pub const SRC1_COLOR: Self = Self(0x88F9);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(0x88FA);
    pub const SRC1_ALPHA: Self = Self(0x8589);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(0x88FB);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Sampler(Uint);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Texture(Uint);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureTarget(Enum);

impl TextureTarget {
    pub const TEXTURE_2D: Self = Self(0x0DE1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct InternalFormat(Enum);
impl InternalFormat {
    pub const R8: Self = Self(0x8229);
    pub const RG8: Self = Self(0x822B);
    pub const RGB8: Self = Self(0x8051);
    pub const RGBA8: Self = Self(0x8058);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureFormat(Enum);
impl TextureFormat {
    pub const RED: Self = Self(0x1903);
    pub const RG: Self = Self(0x8227);
    pub const RGB: Self = Self(0x1907);
    pub const BGR: Self = Self(0x80E0);
    pub const RGBA: Self = Self(0x1908);
    pub const BGRA: Self = Self(0x80E1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureDataFormat(Enum);
impl TextureDataFormat {
    pub const U8: Self = Self(0x1401);
    pub const F32: Self = Self(0x1406);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Border(Int);
impl Border {
    pub const ZERO: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureUnit(Enum);
impl TextureUnit {
    pub const ZERO: Self = Self(0x84C0);

    #[must_use]
    pub const fn new(unit: Enum) -> Self {
        Self(Self::ZERO.0.saturating_add(unit))
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SamplerParameterName(Enum);
impl SamplerParameterName {
    pub const TEXTURE_MAG_FILTER: Self = Self(0x2800);
    pub const TEXTURE_MIN_FILTER: Self = Self(0x2801);
    pub const TEXTURE_WRAP_S: Self = Self(0x2802);
    pub const TEXTURE_WRAP_T: Self = Self(0x2803);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SamplerParameterValue(Int);
impl SamplerParameterValue {
    pub const NEAREST: Self = Self(0x2600);
    pub const LINEAR: Self = Self(0x2601);

    pub const CLAMP_TO_EDGE: Self = Self(0x812F);
    pub const REPEAT: Self = Self(0x2901);
    pub const MIRRORED_REPEAT: Self = Self(0x8370);
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        if value { Self::TRUE } else { Self::FALSE }
    }
}

impl ops::BitOrAssign for ClearMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
