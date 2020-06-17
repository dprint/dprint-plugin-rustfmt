use std::path::PathBuf;
use dprint_core::generate_plugin_code;
use super::config::{Configuration, resolve_config};

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
    super::format_text(file_text, config)
}

generate_plugin_code!();
