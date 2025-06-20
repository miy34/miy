use core::{
    ffi::{self, CStr, c_void},
    fmt::Display,
    mem,
};

use crate as gl;

#[derive(Debug)]
pub struct FunctionLoadingError {
    pub name: &'static str,
}

impl core::error::Error for FunctionLoadingError {}

impl Display for FunctionLoadingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to load function {}", self.name)
    }
}

/// Converts a Rust-like function name &str into the bytes in the OpenGL naming convention
pub const fn to_gl_name(input: &str) -> [u8; 64] {
    let mut buffer = [0; 64];
    let bytes = input.as_bytes();

    buffer[0] = b'g';
    buffer[1] = b'l';

    let mut index = 0;
    let mut buffer_index = 2;
    let mut to_upper = true;

    while index < bytes.len() && bytes[index] != 0 {
        if bytes[index] == b'_' {
            to_upper = true;
        } else {
            buffer[buffer_index] = if to_upper {
                to_upper = false;
                bytes[index].to_ascii_uppercase()
            } else {
                bytes[index]
            };
            buffer_index = buffer_index.saturating_add(1);
        }

        index = index.saturating_add(1);
    }

    buffer
}

macro_rules! gl {
(
        $($name:ident : fn $args:tt $(-> $ret:ty)?),+
) => {
   pub struct Bindings {
    $(
        $name:  extern "system" fn $args $(-> $ret)?,
    )+
    }

    #[expect(clippy::too_many_arguments, reason = "cant change OpenGL's API")]
    impl Bindings{
        #[cfg(feature="context")]
        pub fn with_context(context: &gl::Context) -> Result<Self, FunctionLoadingError> {
            Self::load(|s| context.get_proc_address(s))
        }

        pub fn load(loader: impl Fn(&CStr) -> *const c_void) -> Result<Self, FunctionLoadingError> {
            Ok(Self {
                $(
                    $name: unsafe {
                        load(core::ffi::CStr::from_bytes_until_nul(
                            &to_gl_name(stringify!($name))
                        ).unwrap(),
                        &loader
                    ).unwrap()}
                ),*
            })
        }

    $(
        gl!(@wrapper $name $args $($ret)?);

    )+

    }
};
(@wrapper $name:ident ($($arg:tt : $t:ty),*)) => {
   /// # Safety
   /// OpenGL requires a valid, currently bound context.
   /// The driver situation is madness, and there is UB behaviour
   /// everywhere. Make sure to follow the specs and official documentation
   /// for the actual OpenGL functions, and good luck!
   #[inline]
   pub unsafe fn $name (&self, $($arg: $t),*) {
        (self.$name)($($arg),*);

        if cfg!(debug_assertions) {
            let error = (self.get_error)();
            if error != gl::Error::NO_ERROR {
                panic!("error in {}: {error}", stringify!($name));
            }
        }
    }
};
(@wrapper $name:ident ($($arg:tt : $t:ty),*) $ret:ty) => {
   /// # Safety
   /// OpenGL requires a valid, currently bound context.
   /// The driver situation is madness, and there is UB behaviour
   /// everywhere. Make sure to follow the specs and official documentation
   /// for the actual OpenGL functions, and good luck!
   #[must_use]
   #[inline]
   pub unsafe fn $name (&self, $($arg: $t),*) -> $ret {
        let ret = (self.$name)($($arg),*);
        if cfg!(debug_assertions) {
            let error = (self.get_error)();
            if error != gl::Error::NO_ERROR {
                panic!("error in {}: {error}", stringify!($name));
            }
        }
        ret
    }
};
}

gl!(
    get_error: fn() -> gl::Error,
    //  State
    viewport: fn(x: gl::Int, y: gl::Int, width: gl::Sizei, height: gl::Sizei),
    enable: fn(capability: gl::Capability),
    disable: fn(capability: gl::Capability),
    blend_func: fn(sfactor: gl::BlendFactor, dfactor: gl::BlendFactor),
    get_string: fn(name: gl::Description) -> *const gl::Ubyte,

    // Clear
    clear_color: fn(r: gl::Float, g: gl::Float, b: gl::Float, a: gl::Float),
    clear: fn(mask: gl::ClearMask),

    //  Draw
    draw_arrays: fn(mode: gl::Primitive, first: gl::Start, count: gl::Count),
    draw_elements: fn(mode: gl::Primitive, count: gl::Count, kind: gl::ElementKind, indices: *const c_void),
    //  Vertex Arrays
    gen_vertex_arrays: fn(n: gl::Sizei, arrays: *mut gl::VertexArray),
    bind_vertex_array: fn(array: gl::VertexArray),
    delete_vertex_arrays: fn(n: gl::Sizei, arrays: *const gl::VertexArray),
    //  Buffers
    gen_buffers: fn(n: gl::Sizei, buffers: *mut gl::Buffer),
    bind_buffer: fn(target: gl::BufferTarget, buffer: gl::Buffer),
    buffer_data: fn(target: gl::BufferTarget, size: gl::Sizeiptr, data: *const ffi::c_void, usage: gl::BufferUsage),
    buffer_sub_data: fn(target: gl::BufferTarget, offset: gl::Intptr, size: gl::Sizeiptr, data: *const ffi::c_void),
    bind_buffer_base: fn(target: gl::BufferTarget, index: gl::BufferBinding, buffer: gl::Buffer),
    delete_buffer: fn(buffer: gl::Buffer),

    //  Shaders
    create_shader: fn(kind: gl::ShaderKind) -> gl::Shader,
    shader_source: fn(shader: gl::Shader, count: gl::Sizei, source: *const *const gl::Char, length: *const gl::Int),
    compile_shader: fn(shader: gl::Shader),
    delete_shader: fn(shader: gl::Shader),
    get_shaderiv: fn(shader: gl::Shader, pname: gl::ShaderParameterName, params: *mut gl::Int),
    get_shader_info_log: fn(shader: gl::Shader, max_length: gl::Sizei, length: *mut gl::Sizei, info_log: *mut gl::Char),

    //  ShaderProgram
    create_program: fn() -> gl::Program,
    attach_shader: fn(program: gl::Program, shader: gl::Shader),
    link_program: fn(program: gl::Program),
    detach_shader: fn(program: gl::Program, shader: gl::Shader),
    use_program: fn(program: gl::Program),
    delete_program: fn(program: gl::Program),

    // Texture
    gen_textures: fn(n: gl::Sizei, textures: *mut gl::Texture),
    active_texture: fn(unit: gl::TextureUnit),
    bind_texture: fn(target: gl::TextureTarget, texture: gl::Texture),
    tex_image_2_d: fn(
        target: gl::TextureTarget,
        level: gl::Int,
        internal_format: gl::InternalFormat,
        width: gl::Sizei,
        height: gl::Sizei,
        border: gl::Border,
        format: gl::TextureFormat,
        kind: gl::TextureDataFormat,
        data: *const ffi::c_void
    ),
    generate_mipmap: fn(target: gl::TextureTarget),
    delete_textures: fn(n: gl::Sizei, texture: *const gl::Texture),
    uniform_1i: fn(location: gl::Int, v0: gl::Int),
    //samplers
    gen_samplers: fn(n: gl::Sizei, samplers: *mut gl::Sampler),
    bind_samplers: fn(unit: gl::Uint, sampler: gl::Sampler),
    sampler_parameteri: fn(
        sampler: gl::Sampler,
        pname: gl::SamplerParameterName,
        pvalue: gl::SamplerParameterValue
    ),
    delete_samplers: fn(n: gl::Sizei, samplers: *const gl::Sampler)
);

///
/// # Safety
/// T must be function pointer sized
unsafe fn load<T>(symbol: &CStr, loader: impl Fn(&CStr) -> *const ffi::c_void) -> Option<T> {
    let ptr = loader(symbol);

    // some implementations actually return 1, 2, 3 or -1 on failure
    // see https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions
    if ptr.is_null()
        || ptr as usize == 1
        || ptr as usize == 2
        || ptr as usize == 3
        || ptr as isize == -1
    {
        None
    } else {
        assert_eq!(mem::size_of::<T>(), mem::size_of::<*const ffi::c_void>());

        Some(unsafe { mem::transmute_copy(&ptr) })
    }
}

#[cfg(test)]
mod test {
    use core::ffi::CStr;

    fn compare(name: &str, check: &CStr) {
        let gl_name = super::to_gl_name(name);
        let gl_name = CStr::from_bytes_until_nul(&gl_name).unwrap();
        assert_eq!(gl_name, check);
    }

    #[test]
    fn gl_stringify() {
        compare("ClearColor", c"glClearColor");
        compare("sampler_parameteri", c"glSamplerParameteri");
        compare("tex_image_2_d", c"glTexImage2D");
    }
}
