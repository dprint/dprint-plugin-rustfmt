pub mod configuration;
mod format;
pub use format::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod plugin_handler;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use plugin_handler::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm_plugin;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use wasm_plugin::*;
