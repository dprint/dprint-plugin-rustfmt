use super::configuration::Configuration;

use anyhow::{bail, Result};
use rustfmt_nightly::{Input, Session};

pub fn format_text(file_text: &str, config: &Configuration) -> Result<String> {
  let mut out = Vec::new();
  {
    let input = Input::Text(String::from(file_text));
    let mut session = Session::new(config.rustfmt_config.clone(), Some(&mut out));
    session.format(input)?;
  }

  let text = std::str::from_utf8(&out)?;
  // rustfmt adds this prefix, so just ignore it
  let text = text
    .trim_start_matches("<stdin>:\n\n")
    .trim_start_matches("stdin:\n\n");
  if text.trim().is_empty() && file_text.trim().len() > 0 {
    bail!("Rustfmt had errors.")
  }
  Ok(text.to_string())
}

#[cfg(test)]
mod test {
  use dprint_core::configuration::resolve_global_config;

  use crate::configuration::resolve_config;

  use super::*;

  #[test]
  fn test_format() {
    let global_config = resolve_global_config(Default::default(), &Default::default()).config;
    let config = resolve_config(Default::default(), &global_config).config;
    assert_eq!(format_text("use test;", &config).unwrap(), "use test;\n");
    assert_eq!(format_text("use test;\n", &config).unwrap(), "use test;\n");
    assert_eq!(
      format_text("let test = ...;", &config)
        .err()
        .unwrap()
        .to_string(),
      "Rustfmt had errors."
    );
  }
}
