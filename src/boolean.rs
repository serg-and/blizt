use napi::{JsBoolean, JsUnknown, Result};

use crate::{common::invalid_arg, impl_base_methods, impl_parse_safe_method, BValueBase};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BBoolean {
  base: BValueBase,
  expected: Option<bool>,
}

impl_base_methods!(
  BBoolean,
  Boolean,
  "BBoolean<R | undefined>",
  "BBoolean<R | null>",
  "BBoolean<R | null | undefined>",
  "BBoolean<Exclude<R, undefined>>",
  "BBoolean<Exclude<R, null>>"
);
impl_parse_safe_method!(BBoolean);

#[napi]
impl BBoolean {
  #[napi(ts_return_type = "BBoolean<boolean>")]
  pub fn default() -> BBoolean {
    BBoolean {
      base: BValueBase::default(),
      expected: None,
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, value: JsUnknown) -> Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    let js_bool: JsBoolean = value.try_into()?;
    let b = js_bool.get_value()?;

    if let Some(expected) = self.expected {
      if expected != b {
        return Err(invalid_arg(
          format!("Boolean did not match expected value '{}'", expected).as_str(),
        ));
      }
    }

    Ok(js_bool.into_unknown())
  }

  #[napi(ts_return_type = "BBoolean<R>")]
  pub fn is_true(&self) -> Self {
    BBoolean {
      expected: Some(true),
      ..self.clone()
    }
  }

  #[napi(ts_return_type = "BBoolean<R>")]
  pub fn is_false(&self) -> Self {
    BBoolean {
      expected: Some(false),
      ..self.clone()
    }
  }
}
