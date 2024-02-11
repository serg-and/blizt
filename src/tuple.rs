use napi::{bindgen_prelude::ClassInstance, Env, Error, JsObject, JsUnknown, Result, Status};

use crate::{
  common::{invalid_arg, parse_btype},
  impl_base_methods, impl_parse_safe_method_with_env, BType, BValueBase, BWrapped,
};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BTuple {
  base: BValueBase,
  inner: Vec<BType>,
}

impl_base_methods!(
  BTuple,
  Tuple,
  "BTuple<R | undefined>",
  "BTuple<R | null>",
  "BTuple<R | null | undefined>",
  "BTuple<Exclude<R, undefined>>",
  "BTuple<Exclude<R, null>>"
);
impl_parse_safe_method_with_env!(BTuple);

#[napi]
impl BTuple {
  /// For internal use only!
  #[napi(js_name = "_fromWrapped", ts_return_type = "unknown")]
  pub fn from_wrapped(wrapped: Vec<ClassInstance<BWrapped>>) -> BTuple {
    let inner: Vec<BType> = wrapped.into_iter().map(|w| w.clone().inner).collect();
    BTuple {
      base: BValueBase::default(),
      inner,
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, env: Env, value: JsUnknown) -> Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    let obj: JsObject = value.try_into()?;
    if !obj.is_array()? {
      return Err(Error::new(Status::ArrayExpected, "Array expected"));
    }

    let expected_len = self.inner.len();
    let val_len = obj.get_array_length()?;
    if val_len != expected_len as u32 {
      return Err(invalid_arg(
        format!(
          "Tuple length ({}) does not equal expected length ({})",
          val_len, expected_len
        )
        .as_str(),
      ));
    }

    let mut array = env.create_array_with_length(expected_len)?;

    for (i, b_type) in self.inner.iter().enumerate() {
      let el = obj.get_element::<JsUnknown>(i as u32)?;
      let parsed = parse_btype(&b_type, el, env)?;
      array.set_element(i as u32, parsed)?;
    }

    Ok(array.into_unknown())
  }
}
