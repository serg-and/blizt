use napi::{bindgen_prelude::ClassInstance, Env, Error, JsUnknown, Result, Status};

use crate::{
  common::{invalid_arg, parse_btype, ClonableJsUnknown},
  impl_base_methods, impl_parse_safe_method_with_env, BType, BValueBase, BWrapped,
};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BUnion {
  base: BValueBase,
  inner: Vec<BType>,
}

impl_base_methods!(
  BUnion,
  Union,
  "BUnion<R | undefined>",
  "BUnion<R | null>",
  "BUnion<R | null | undefined>",
  "BUnion<Exclude<R, undefined>>",
  "BUnion<Exclude<R, null>>"
);
impl_parse_safe_method_with_env!(BUnion);

#[napi]
impl BUnion {
  /// For internal use only!
  #[napi(js_name = "_fromWrapped", ts_return_type = "unknown")]
  pub fn from_wrapped(wrapped: Vec<ClassInstance<BWrapped>>) -> BUnion {
    let inner: Vec<BType> = wrapped.into_iter().map(|w| w.clone().inner).collect();
    BUnion {
      base: BValueBase::default(),
      inner,
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, env: Env, value: JsUnknown) -> Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    // // TODO: idk if using Rc is necessary here, would prefer to just clone value, but JsUnknown does not implement Clone
    // let rc = Rc::new(value);
    // for b_type in &self.inner {
    //   match parse_btype_rc(&b_type, rc.clone(), env) {
    //     Ok(r) => return Ok(r),
    //     Err(Error { status: Status:: InvalidArg, .. }) => continue, // invalid value for BType
    //     Err(err) => return Err(err),
    //   };
    // }

    // TODO: this seems like the best way for now to clone JsValues
    let clonable = ClonableJsUnknown::new(env, value)?;

    for b_type in &self.inner {
      match parse_btype(&b_type, clonable.create_clone()?, env) {
        Ok(r) => return Ok(r),
        // invalid value for BType, continue to next type to check
        Err(Error {
          status: Status::InvalidArg,
          ..
        }) => continue,
        Err(err) => return Err(err),
      };
    }

    Err(invalid_arg("Value did not match any of the Union types"))
  }

  #[napi(ts_return_type = "BUnion<R | T>")]
  pub fn merge(&self, #[napi(ts_arg_type = "BUnion<T>")] schema: ClassInstance<BUnion>) -> BUnion {
    let mut others = schema.clone().inner;
    let mut clone = self.clone();
    clone.inner.append(&mut others);

    BUnion { ..clone }
  }
}
