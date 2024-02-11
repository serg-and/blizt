use std::collections::HashMap;

use napi::{bindgen_prelude::ClassInstance, Env, JsObject, JsUnknown, Result, ValueType};

use crate::{
  common::{get_btype_base, invalid_arg, parse_btype},
  impl_base_methods, impl_parse_safe_method_with_env, BType, BValueBase, BWrapped,
};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BObject {
  base: BValueBase,
  inner: HashMap<String, BType>,
}

impl_base_methods!(
  BObject,
  Object,
  "BObject<R | undefined>",
  "BObject<R | null>",
  "BObject<R | null | undefined>",
  "BObject<Exclude<R, undefined>>",
  "BObject<Exclude<R, null>>"
);
impl_parse_safe_method_with_env!(BObject);

#[napi]
impl BObject {
  /// For internal use only!
  #[napi(ts_return_type = "BObject<unknown>")]
  pub fn new(schema: HashMap<String, ClassInstance<BWrapped>>) -> BObject {
    BObject {
      base: BValueBase::default(),
      inner: schema
        .iter()
        .map(|(k, v)| (k.clone(), v.inner.clone()))
        .collect(),
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, env: Env, value: JsUnknown) -> Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    let obj: JsObject = value.try_into()?;
    let mut res_obj = env.create_object()?;

    for (key, b_type) in &self.inner {
      let property = obj.get_named_property::<JsUnknown>(&key)?;

      // key is not present, skip if type is optional, otherwise throw error
      if property.get_type()? == ValueType::Undefined {
        if get_btype_base(&b_type).optional {
          continue;
        } else {
          return Err(invalid_arg(
            format!("Object missing required key '{}'", &key).as_str(),
          ));
        }
      }

      let v = parse_btype(&b_type, property, env)?;
      res_obj.set_named_property(&key, v)?;
    }

    Ok(res_obj.into_unknown())
  }

  /// Create a new object schema by merging 2 objects.
  /// Optional and Nullable parameters of both objects are ignored for new object.
  #[napi(
    ts_return_type = "BObject<Omit<Exclude<R, null | undefined>, keyof Exclude<T, null | undefined>> & Exclude<T, null | undefined>>"
  )]
  pub fn merge(
    &self,
    #[napi(ts_arg_type = "BObject<T>")] schema: ClassInstance<BObject>,
  ) -> BObject {
    let others = schema.clone().inner;
    let mut clone = self.clone();
    clone.inner.extend(others);
    clone.base.optional = false;
    clone.base.nullable = false;

    BObject { ..clone }
  }
}
