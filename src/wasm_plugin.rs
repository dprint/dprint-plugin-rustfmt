// Building for Wasm unfortunately currently doesn't work.
// See https://github.com/rust-lang/rustfmt/issues/4845
use super::RustFmtPluginHandler;
use dprint_core::generate_plugin_code;

generate_plugin_code!(RustFmtPluginHandler, RustFmtPluginHandler::new());
