use crate::configuration::resolve_config;

use super::configuration::Configuration;

use anyhow::bail;
use anyhow::Result;
use rustfmt_nightly::Input;
use rustfmt_nightly::Session;

pub fn format_text(file_text: String, config: &Configuration) -> Result<Option<String>> {
  let original_text = file_text.clone();
  let mut out = Vec::new();
  {
    let input = Input::Text(file_text);
    // config is not `Sync`, so unfortunately we have to recreate it each time
    let (config, _) = resolve_config(&config.config, &config.global_config);
    let mut session = Session::new(config, Some(&mut out));
    session.format(input)?;
  }

  let text = std::str::from_utf8(&out)?;
  // rustfmt adds this prefix, so just ignore it
  let text = text
    .trim_start_matches("<stdin>:\n\n")
    .trim_start_matches("stdin:\n\n");
  if text.trim().is_empty() && original_text.trim().len() > 0 {
    bail!("Rustfmt had errors.")
  }
  if original_text == text {
    Ok(None)
  } else {
    Ok(Some(text.to_string()))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_format() {
    let config = Configuration {
        config: Default::default(),
        global_config: Default::default(),
    };
    assert_eq!(format_text("use test;".to_string(), &config).unwrap(), Some("use test;\n".to_string()));
    assert_eq!(format_text("use test;\n".to_string(), &config).unwrap(), None);
    assert_eq!(
      format_text("let test = ...;".to_string(), &config)
        .err()
        .unwrap()
        .to_string(),
      "Rustfmt had errors."
    );
  }
}
