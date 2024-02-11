use napi::{JsUndefined, JsUnknown, Result};

use crate::{impl_base_methods, impl_parse_safe_method, BValueBase};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BUndefined {
  base: BValueBase,
}

impl_base_methods!(
  BUndefined,
  Undefined,
  "BUndefined<R | undefined>",
  "BUndefined<R | null>",
  "BUndefined<R | null | undefined>",
  "BUndefined<Exclude<R, undefined>>",
  "BUndefined<Exclude<R, null>>"
);
impl_parse_safe_method!(BUndefined);

#[napi]
impl BUndefined {
  #[napi(ts_return_type = "BUndefined<undefined>")]
  pub fn default() -> BUndefined {
    BUndefined {
      base: BValueBase {
        optional: true,
        nullable: false,
      },
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, value: JsUnknown) -> Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    let js_undefined: JsUndefined = value.try_into()?;
    Ok(js_undefined.into_unknown())
  }
}
