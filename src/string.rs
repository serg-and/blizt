use napi::{JsString, JsUnknown};

use crate::{common::invalid_arg, impl_base_methods, impl_parse_safe_method, BValueBase};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BString {
  base: BValueBase,
  len: Option<u32>,
  min: Option<u32>,
  max: Option<u32>,
}

impl_base_methods!(
  BString,
  String,
  "BString<R | undefined>",
  "BString<R | null>",
  "BString<R | null | undefined>",
  "BString<Exclude<R, undefined>>",
  "BString<Exclude<R, null>>"
);
impl_parse_safe_method!(BString);

#[napi]
impl BString {
  #[napi(ts_return_type = "BString<string>")]
  pub fn default() -> BString {
    BString {
      base: BValueBase::default(),
      len: None,
      min: None,
      max: None,
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, value: JsUnknown) -> napi::Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    let js_string: JsString = value.try_into()?;
    // TODO: unsure if using utf16 or utf8 is better, speeds differs based on input;
    let js_utf8 = js_string.into_utf8()?;
    let str = js_utf8.as_str()?;
    let length = str.len() as u32;

    if let Some(expected) = self.len {
      if expected != length {
        return Err(invalid_arg(
          format!(
            "String length ({}) does not equal expected length ({})",
            length, expected
          )
          .as_str(),
        ));
      }
    } else {
      if let Some(min) = self.min {
        if length < min {
          return Err(invalid_arg("String length is lower than minimum"));
        }
      }
      if let Some(max) = self.max {
        if length > max {
          return Err(invalid_arg("String length is higher than maximum"));
        }
      }
    }

    Ok(js_string.into_unknown())
  }

  #[napi]
  pub fn length(&self, value: u32) -> Self {
    BString {
      len: Some(value),
      ..self.clone()
    }
  }

  #[napi]
  pub fn min(&self, value: u32) -> Self {
    BString {
      min: Some(value),
      ..self.clone()
    }
  }

  #[napi]
  pub fn max(&self, value: u32) -> Self {
    BString {
      max: Some(value),
      ..self.clone()
    }
  }
}
