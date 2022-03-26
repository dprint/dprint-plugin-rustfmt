use super::configuration::resolve_config;
use super::configuration::Configuration;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::plugins::AsyncPluginHandler;
use dprint_core::plugins::BoxFuture;
use dprint_core::plugins::FormatRequest;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::Host;
use dprint_core::plugins::PluginInfo;
use std::sync::Arc;

pub struct RustFmtPluginHandler;

impl AsyncPluginHandler for RustFmtPluginHandler {
  type Configuration = Configuration;

  fn plugin_info(&self) -> PluginInfo {
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

  fn license_text(&self) -> String {
    include_str!("../LICENSE").to_string()
  }

  fn resolve_config(
    &self,
    config: ConfigKeyMap,
    global_config: GlobalConfiguration,
  ) -> ResolveConfigurationResult<Configuration> {
    let (_, diagnostics) = resolve_config(&config, &global_config);
    ResolveConfigurationResult { diagnostics, config: Configuration {
      config,
      global_config,
    } }
  }

  fn format(
    &self,
    request: FormatRequest<Self::Configuration>,
    _host: Arc<dyn Host>,
  ) -> BoxFuture<FormatResult> {
    Box::pin(async move {
      // range formatting not supported
      if request.range.is_some() {
        return Ok(None);
      }

      super::format::format_text(request.file_text, &request.config)
    })
  }
}
