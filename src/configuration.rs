use rustfmt_nightly::{Config, Edition, EmitMode, NewlineStyle};

use dprint_core::configuration::{
  ConfigKeyMap, ConfigKeyValue, ConfigurationDiagnostic, GlobalConfiguration, NewLineKind,
  ResolveConfigurationResult,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Configuration {
  // Unfortunately no resolved configuration at the moment because serializing
  // rustfmt's PartialConfig configuration kept causing a panic
  #[serde(flatten)]
  pub(crate) config: ConfigKeyMap,
  #[serde(skip_serializing, skip_deserializing)]
  pub(crate) rustfmt_config: Config,
}

pub fn resolve_config(
  config: ConfigKeyMap,
  global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
  let mut rustfmt_config = Config::default();
  let mut diagnostics = Vec::new();

  rustfmt_config.set().edition(Edition::Edition2018);

  // set dprint global configuration
  if let Some(line_width) = global_config.line_width {
    rustfmt_config.set().max_width(line_width as usize);
  }
  if let Some(use_tabs) = global_config.use_tabs {
    rustfmt_config.set().hard_tabs(use_tabs);
  }
  if let Some(indent_width) = global_config.indent_width {
    rustfmt_config.set().tab_spaces(indent_width as usize);
  }

  rustfmt_config
    .set()
    .newline_style(match global_config.new_line_kind {
      Some(NewLineKind::Auto) => NewlineStyle::Auto,
      Some(NewLineKind::LineFeed) | None => NewlineStyle::Unix,
      Some(NewLineKind::CarriageReturnLineFeed) => NewlineStyle::Windows,
      Some(NewLineKind::System) => NewlineStyle::Native,
    });

  for (key, value) in config.iter() {
    if key == "newLineKind" {
      match value {
        ConfigKeyValue::String(value) => match value.as_str() {
          "auto" => rustfmt_config.set().newline_style(NewlineStyle::Auto),
          "lf" => rustfmt_config.set().newline_style(NewlineStyle::Unix),
          "crlf" => rustfmt_config.set().newline_style(NewlineStyle::Windows),
          "system" => rustfmt_config.set().newline_style(NewlineStyle::Native),
          _ => {
            diagnostics.push(ConfigurationDiagnostic {
              property_name: String::from(key),
              message: format!("Invalid newline kind: {}", value),
            });
          }
        },
        _ => {
          diagnostics.push(ConfigurationDiagnostic {
            property_name: String::from(key),
            message: String::from("Newline kind must be a string."),
          });
        }
      }
      continue;
    }

    let key = match key.as_str() {
      "lineWidth" => "max_width",
      "useTabs" => "hard_tabs",
      "indentWidth" => "tab_spaces",
      _ => key,
    };
    let value = key_value_to_string(value);
    if Config::is_valid_key_val(key, &value) {
      rustfmt_config.override_value(key, &value);
    } else {
      let message = format!(
        "Invalid key or value in configuration. Key: {}, Value: {}",
        key, value
      );
      diagnostics.push(ConfigurationDiagnostic {
        property_name: String::from(key),
        message,
      });
    }
  }

  rustfmt_config.set().emit_mode(EmitMode::Stdout);

  ResolveConfigurationResult {
    diagnostics,
    config: Configuration {
      config,
      rustfmt_config,
    },
  }
}

fn key_value_to_string(value: &ConfigKeyValue) -> String {
  match value {
    ConfigKeyValue::String(value) => value.clone(),
    ConfigKeyValue::Number(value) => value.to_string(),
    ConfigKeyValue::Bool(value) => value.to_string(),
  }
}
