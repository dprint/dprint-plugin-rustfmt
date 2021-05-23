use std::path::Path;
use dprint_core::types::ErrBox;
use dprint_core::configuration::{GlobalConfiguration, ResolveConfigurationResult, ConfigKeyMap};
use dprint_core::plugins::{PluginHandler, PluginInfo};
use super::configuration::{Configuration, resolve_config};

pub struct RustFmtPluginHandler {}

impl RustFmtPluginHandler {
    pub const fn new() -> Self {
        RustFmtPluginHandler {}
    }
}

impl PluginHandler<Configuration> for RustFmtPluginHandler {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        resolve_config(config, global_config)
    }

    fn get_plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "rustfmt".to_string(),
            file_extensions: vec!["rs".to_string()],
            help_url: "https://dprint.dev/plugins/rustfmt".to_string(),
            config_schema_url: "".to_string()
        }
    }

    fn get_license_text(&mut self) -> String {
        std::str::from_utf8(include_bytes!("../LICENSE")).unwrap().into()
    }

    fn format_text(
        &mut self,
        _: &Path,
        file_text: &str,
        config: &Configuration,
        _: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String, ErrBox>,
    ) -> Result<String, ErrBox> {
        super::format_text(file_text, config)
    }
}
