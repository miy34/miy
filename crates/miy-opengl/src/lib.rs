#![no_std]

extern crate alloc;

mod bindings;
mod types;

#[cfg(feature = "context")]
mod context;

pub mod util;

pub use bindings::Bindings;
pub use types::*;

#[cfg(feature = "context")]
pub use context::{Context, ContextWindow, Profile, SurfaceSize, Version};
