use rustfmt_nightly::{Input, Session};
use super::config::{Configuration};

pub fn format_text(
    file_text: &str,
    config: &Configuration,
) -> Result<String, String> {
    let mut out = Vec::new();
    {
        let input = Input::Text(String::from(file_text));
        let mut session = Session::new(config.rustfmt_config.clone(), Some(&mut out));
        match session.format(input) {
            Err(err) => {
                return Err(err.to_string());
            },
            _ => {
                // do nothing
            }
        }
    }

    // rustfmt adds this prefix, so just ignore it
    let prefix = "stdin:\n\n";
    Ok(String::from(std::str::from_utf8(&out[prefix.len()..]).unwrap()))
}
