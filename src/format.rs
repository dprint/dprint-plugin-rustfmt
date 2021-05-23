use rustfmt_nightly::{Input, Session};
use dprint_core::types::ErrBox;
use super::configuration::Configuration;

pub fn format_text(
    file_text: &str,
    config: &Configuration,
) -> Result<String, ErrBox> {
    let mut out = Vec::new();
    {
        let input = Input::Text(String::from(file_text));
        let mut session = Session::new(config.rustfmt_config.clone(), Some(&mut out));
        session.format(input)?;
    }

    // rustfmt adds this prefix, so just ignore it
    let prefix = "stdin:\n\n";
    Ok(String::from(std::str::from_utf8(&out[prefix.len()..]).unwrap()))
}
