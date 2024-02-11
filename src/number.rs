use napi::{JsNumber, JsUnknown, Result};

use crate::{common::invalid_arg, impl_base_methods, impl_parse_safe_method, BValueBase};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BNumber {
  base: BValueBase,
  min: Option<f64>,
  max: Option<f64>,
}

impl_base_methods!(
  BNumber,
  Number,
  "BNumber<R | undefined>",
  "BNumber<R | null>",
  "BNumber<R | null | undefined>",
  "BNumber<Exclude<R, undefined>>",
  "BNumber<Exclude<R, null>>"
);
impl_parse_safe_method!(BNumber);

#[napi]
impl BNumber {
  #[napi(ts_return_type = "BNumber<number>")]
  pub fn default() -> BNumber {
    BNumber {
      base: BValueBase::default(),
      min: None,
      max: None,
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, value: JsUnknown) -> Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    let js_number: JsNumber = value.try_into()?;
    let n = js_number.get_double()?;

    if let Some(min) = self.min {
      if n < min {
        return Err(invalid_arg("Number is lower than minimum"));
      }
    }
    if let Some(max) = self.max {
      if n > max {
        return Err(invalid_arg("Number is higher than maximum"));
      }
    }

    Ok(js_number.into_unknown())
  }

  #[napi(ts_return_type = "BNumber<R>")]
  pub fn min(&self, min: f64) -> Self {
    BNumber {
      min: Some(min),
      ..self.clone()
    }
  }

  #[napi(ts_return_type = "BNumber<R>")]
  pub fn max(&self, max: f64) -> Self {
    BNumber {
      max: Some(max),
      ..self.clone()
    }
  }

  #[napi(ts_return_type = "BNumber<R>")]
  pub fn positive(&self) -> Self {
    BNumber {
      min: Some(0.),
      ..self.clone()
    }
  }

  #[napi(ts_return_type = "BNumber<R>")]
  pub fn negative(&self) -> Self {
    BNumber {
      max: Some(0.),
      ..self.clone()
    }
  }
}
