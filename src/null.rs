use napi::{JsNull, JsUnknown, Result};

use crate::{impl_base_methods, impl_parse_safe_method, BValueBase};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BNull {
  base: BValueBase,
}

impl_base_methods!(
  BNull,
  Null,
  "BNull<R | undefined>",
  "BNull<R | null>",
  "BNull<R | null | undefined>",
  "BNull<Exclude<R, undefined>>",
  "BNull<Exclude<R, null>>"
);
impl_parse_safe_method!(BNull);

#[napi]
impl BNull {
  #[napi(ts_return_type = "BNull<null>")]
  pub fn default() -> BNull {
    BNull {
      base: BValueBase {
        optional: false,
        nullable: true,
      },
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, value: JsUnknown) -> Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    let js_undefined: JsNull = value.try_into()?;
    Ok(js_undefined.into_unknown())
  }
}
