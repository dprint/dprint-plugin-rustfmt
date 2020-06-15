use rustfmt_nightly::{Input, Session, Config};

use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use dprint_core::generate_plugin_code;
use dprint_core::configuration::{GlobalConfiguration, ResolveConfigurationResult, get_unknown_property_diagnostics};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Configuration {
    // add configuration properties here
}

fn resolve_config(
    config: HashMap<String, String>,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {

    ResolveConfigurationResult {
        diagnostics: Vec::new(),
        config: Configuration {}
    }
}

fn get_plugin_config_key() -> String {
    // return the JSON object key name used in the configuration file
    String::from("rustfmt")
}

fn get_plugin_file_extensions() -> Vec<String> {
    vec![String::from("rs")]
}

fn get_plugin_help_url() -> String {
    String::from("https://dprint.dev/plugins/rustfmt")
}

fn get_plugin_config_schema_url() -> String {
    // for now, return an empty string. Return a schema url once VSCode
    // supports $schema properties in descendant objects:
    // https://github.com/microsoft/vscode/issues/98443
    String::new()
}

fn get_plugin_license_text() -> String {
    std::str::from_utf8(include_bytes!("../LICENSE")).unwrap().into()
}

fn format_text(
    _: &PathBuf,
    file_text: &str,
    config: &Configuration,
) -> Result<String, String> {
    use rustfmt_nightly::{EmitMode, Edition};

    let mut out = Vec::with_capacity(file_text.len());
    {
        let input = Input::Text(String::from(file_text));
        let mut config = Config::default();
        config.set().emit_mode(EmitMode::Stdout);
        config.set().edition(Edition::Edition2018);
        let mut session = Session::new(config, Some(&mut out));
        session.format(input).unwrap();
    }
    Ok(String::from_utf8(out).unwrap())
}

generate_plugin_code!();
