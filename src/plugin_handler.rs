use super::configuration::{resolve_config, Configuration};
use anyhow::Result;
use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult};
use dprint_core::plugins::{PluginHandler, PluginInfo};
use std::path::Path;

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
      file_names: vec![],
      help_url: "https://dprint.dev/plugins/rustfmt".to_string(),
      config_schema_url: "".to_string(),
      update_url: Some(
        "https://plugins.dprint.dev/dprint/dprint-plugin-rustfmt/latest.json".to_string(),
      ),
    }
  }

  fn get_license_text(&mut self) -> String {
    std::str::from_utf8(include_bytes!("../LICENSE"))
      .unwrap()
      .into()
  }

  fn format_text(
    &mut self,
    _: &Path,
    file_text: &str,
    config: &Configuration,
    _: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String>,
  ) -> Result<String> {
    super::format_text(file_text, config)
  }
}
