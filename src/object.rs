use std::collections::HashMap;

use napi::{bindgen_prelude::ClassInstance, Env, JsObject, JsUnknown, Result};

use crate::{
  common::{get_btype_base, invalid_arg, parse_btype},
  impl_base_methods, impl_parse_safe_method_with_env, BType, BValueBase, BWrapped,
};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BObject {
  base: BValueBase,
  inner: HashMap<String, BType>,
  strict: bool,
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
      strict: false,
    }
  }

  #[napi(ts_return_type = "R")]
  pub fn parse(&self, env: Env, value: JsUnknown) -> Result<JsUnknown> {
    if self.base.skip_parse(&value)? {
      return Ok(value);
    }

    let mut obj: JsObject = value.try_into()?;
    let mut obj_keys = JsObject::keys(&obj)?;

    for (key, b_type) in &self.inner {
      let position = obj_keys.iter().position(|r| r == key);
      if position.is_none() {
        if get_btype_base(&b_type).optional {
          continue;
        } else {
          return Err(invalid_arg(
            format!("Object missing required key '{}'", &key).as_str(),
          ));
        }
      }

      obj_keys.remove(position.unwrap());

      let property = obj.get_named_property_unchecked::<JsUnknown>(&key)?;
      parse_btype(&b_type, property, env)?;
    }

    // obj has unknown keys, error if strict, otherwise remove keys
    if obj_keys.len() != 0 {
      if self.strict {
        return Err(invalid_arg(
          format!(
            "Unknown key(s) [{}] in strict object",
            obj_keys.into_iter().collect::<Vec<String>>().join(", ")
          )
          .as_str(),
        ));
      }

      for key in obj_keys {
        obj.delete_named_property(key.as_str())?;
      }
    }

    Ok(obj.into_unknown())
  }

  // #[napi(ts_return_type = "R")]
  // pub fn parse_unsafe_potential(&self, env: Env, value: JsUnknown) -> Result<JsUnknown> {
  //   if self.base.skip_parse(&value)? {
  //     return Ok(value);
  //   }

  //   let obj: JsObject = value.try_into()?;

  //   for (key, b_type) in &self.inner {
  //     let property = obj.get_named_property_unchecked::<JsUnknown>(&key)?;

  //     // key is not present, skip if type is optional, otherwise throw error
  //     if property.get_type()? == ValueType::Undefined {
  //       if get_btype_base(&b_type).optional {
  //         continue;
  //       } else {
  //         return Err(invalid_arg(
  //           format!("Object missing required key '{}'", &key).as_str(),
  //         ));
  //       }
  //     }

  //     parse_btype(&b_type, property, env)?;
  //   }

  //   Ok(obj.into_unknown())
  // }

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

  /// Strict does not allow unknown keys when parsing.
  /// When not strict onknown object keys are removed from the value, when strict parsing fails.
  #[napi(ts_return_type = "BObject<R>")]
  pub fn strict(&self) -> BObject {
    BObject {
      strict: true,
      ..self.clone()
    }
  }

  /// Removes strict option.
  #[napi(ts_return_type = "BObject<R>")]
  pub fn un_strict(&self) -> BObject {
    BObject {
      strict: false,
      ..self.clone()
    }
  }
}

// pub struct Measure {
//   name: String,
//   start: Instant,
// }

// impl Measure {
//   pub fn elapsed(&self) {
//     println!("{}: {}ns", self.name, self.start.elapsed().as_nanos());
//   }
// }

// pub fn measure(name: &str) -> Measure {
//   Measure {
//     name: name.to_string(),
//     start: Instant::now(),
//   }
// }
