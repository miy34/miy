use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use core::num::NonZeroU32;

use glutin::{
    config::{Api, ConfigTemplateBuilder, GlConfig},
    context::{
        ContextAttributesBuilder, GlProfile, PossiblyCurrentContext, Version as glutinVersion,
    },
    display::{Display, DisplayApiPreference},
    prelude::{GlDisplay as _, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface},
};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::types;

pub struct Context {
    _window: Box<dyn ContextWindow>,
    surface: Surface<WindowSurface>,
    display: Display,
    context: PossiblyCurrentContext,
    surface_size: SurfaceSize,
}

pub trait ContextWindow: HasDisplayHandle + HasWindowHandle + 'static {}
impl<T> ContextWindow for T where T: HasDisplayHandle + HasWindowHandle + 'static {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SurfaceSize {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
}

#[derive(Debug)]
pub enum ContextError {
    InvalidDisplay(String),
    FailedToCreateContext(String),
    FailedToCreateSurface(String),
    FailedToMakeCurrent(String),
    FailedToSetSwapInterval(String),
}

impl core::error::Error for ContextError {}

impl core::fmt::Display for ContextError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ContextError::InvalidDisplay(e) => write!(f, "invalid display, caused by {e}"),
            ContextError::FailedToCreateContext(e) => {
                write!(f, "failed to create context, caused by {e}")
            }
            ContextError::FailedToCreateSurface(e) => {
                write!(f, "failed to create surface, caused by {e}")
            }
            ContextError::FailedToMakeCurrent(e) => {
                write!(f, "failed to make context current, caused by {e}")
            }
            ContextError::FailedToSetSwapInterval(e) => {
                write!(f, "failed to set swap interval, caused by {e}")
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Version {
    GL43,
    GL33,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Profile {
    Core,
    Compatibility,
}

impl Version {
    pub fn major(&self) -> u8 {
        match self {
            Version::GL43 => 4,
            Version::GL33 => 3,
        }
    }

    pub fn minor(&self) -> u8 {
        match self {
            Version::GL43 | Version::GL33 => 3,
        }
    }
}

impl Context {
    //// Create an OpenGL context and makes it current
    pub fn new<W>(
        version: Version,
        profile: Profile,
        surface_size: SurfaceSize,
        window: W,
    ) -> Result<Self, ContextError>
    where
        W: ContextWindow,
    {
        let raw_window_handle = window.window_handle().unwrap().as_raw();
        let raw_display_handle = window.display_handle().unwrap().as_raw();

        #[cfg(target_os = "windows")]
        let display = Display::new(
            raw_display_handle,
            DisplayApiPreference::Wgl(Some(raw_window_handle)),
        )
        .map_err(|e| ContextError::InvalidDisplay(e.to_string()))?;

        #[cfg(target_os = "linux")]
        let display = unsafe {
            Display::new(raw_display_handle, DisplayApiPreference::Egl)
                .map_err(|e| ContextError::InvalidDisplay(e.to_string()))
        }?;

        let config_template = ConfigTemplateBuilder::default()
            .with_api(Api::OPENGL)
            .build();

        let config = unsafe {
            display
                .find_configs(config_template)
                .unwrap()
                .find(|v| v.srgb_capable() && v.hardware_accelerated())
                .ok_or(ContextError::InvalidDisplay(
                    "no hardware accelerated and srgb capable display found".into(),
                ))?
        };

        let version = glutinVersion {
            major: version.major(),
            minor: version.minor(),
        };

        let context_attributes = ContextAttributesBuilder::new()
            .with_debug(true)
            .with_profile(if profile == Profile::Core {
                GlProfile::Core
            } else {
                GlProfile::Compatibility
            })
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(version)))
            .build(Some(raw_window_handle));

        let context = unsafe {
            display
                .create_context(&config, &context_attributes)
                .map_err(|e| ContextError::FailedToCreateContext(e.to_string()))?
        };

        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new()
            .with_srgb(Some(true))
            .build(raw_window_handle, surface_size.width, surface_size.height);

        let surface = unsafe {
            display
                .create_window_surface(&config, &surface_attributes)
                .map_err(|e| ContextError::FailedToCreateSurface(e.to_string()))?
        };

        let context = context
            .make_current(&surface)
            .map_err(|e| ContextError::FailedToMakeCurrent(e.to_string()))?;

        log::info!("{}", display.version_string());

        surface
            .set_swap_interval(&context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
            .map_err(|e| ContextError::FailedToSetSwapInterval(e.to_string()))?;

        Ok(Self {
            _window: Box::new(window),
            surface,
            display,
            context,
            surface_size,
        })
    }

    pub fn get_proc_address(&self, symbol: &core::ffi::CStr) -> *const core::ffi::c_void {
        self.display.get_proc_address(symbol)
    }

    pub fn vsync(&self, enable: bool) {
        let _ = self.surface.set_swap_interval(
            &self.context,
            if enable {
                SwapInterval::Wait(NonZeroU32::new(1).unwrap())
            } else {
                SwapInterval::DontWait
            },
        );
    }

    pub fn make_current(&self) {
        let _ = self.context.make_current(&self.surface);
    }

    pub fn swap_buffers(&self) {
        let _ = self.surface.swap_buffers(&self.context);
    }

    pub fn resize_surface(&mut self, new_size: SurfaceSize) {
        if self.surface_size == new_size {
            return;
        }

        self.surface_size = new_size;
        self.surface
            .resize(&self.context, new_size.width, new_size.height);
    }

    pub fn surface_size(&self) -> SurfaceSize {
        self.surface_size
    }
}

impl SurfaceSize {
    pub fn new(width: u32, height: u32) -> Option<Self> {
        Some(Self {
            width: NonZeroU32::new(width)?,
            height: NonZeroU32::new(height)?,
        })
    }

    pub fn as_int(&self) -> (types::Int, types::Int) {
        (
            self.width.get() as types::Int,
            self.height.get() as types::Int,
        )
    }
}
