use napi::{bindgen_prelude::ClassInstance, Env, Error, JsObject, JsUnknown, Result, Status};

use crate::{
  common::{invalid_arg, parse_btype},
  impl_base_methods, impl_parse_safe_method_with_env, BType, BValueBase, BWrapped,
};

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BArray {
  base: BValueBase,
  inner: Box<BType>,
  len: Option<u32>,
  min: Option<u32>,
  max: Option<u32>,
}

impl_base_methods!(
  BArray,
  Array,
  "BArray<R | undefined>",
  "BArray<R | null>",
  "BArray<R | null | undefined>",
  "BArray<Exclude<R, undefined>>",
  "BArray<Exclude<R, null>>"
);
impl_parse_safe_method_with_env!(BArray);

#[napi]
impl BArray {
  /// For internal use only!
  #[napi(js_name = "_fromWrapped", ts_return_type = "BArray<unknown>")]
  pub fn from_wrapped(wrapped: ClassInstance<BWrapped>) -> BArray {
    BArray {
      base: BValueBase::default(),
      inner: Box::new(wrapped.clone().inner),
      len: None,
      min: None,
      max: None,
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

    let length = obj.get_array_length()?;

    if let Some(expected) = self.len {
      if expected != length {
        return Err(invalid_arg(
          format!(
            "Array length ({}) does not equal expected length ({})",
            length, expected
          )
          .as_str(),
        ));
      }
    } else {
      if let Some(min) = self.min {
        if length < min {
          return Err(invalid_arg("Array length is lower than minimum"));
        }
      }
      if let Some(max) = self.max {
        if length > max {
          return Err(invalid_arg("Array length is higher than maximum"));
        }
      }
    }

    // Ok(
    //     (0..length)
    //         .map(|i| {
    //         let el = obj.get_element::<JsUnknown>(i)?;
    //         Ok(match &*self.inner {
    //             BType::Boolean(s) => s.parse(el)?,
    //             BType::Number(s) => s.parse(el)?,
    //             _ => Err(invalid_arg("idk"))?,
    //         })
    //     })
    //     .collect::<Result<Vec<JsUnknown>>>()?,
    // )

    let mut array = env.create_array_with_length(length as usize)?;

    for i in 0..length {
      let el = obj.get_element::<JsUnknown>(i)?;
      let parsed = parse_btype(&self.inner, el, env)?;
      array.set_element(i, parsed)?;
    }

    Ok(array.into_unknown())
  }

  #[napi(ts_return_type = "BArray<R>")]
  pub fn length(&self, value: u32) -> Self {
    BArray {
      len: Some(value),
      ..self.clone()
    }
  }

  #[napi(ts_return_type = "BArray<R>")]
  pub fn min(&self, value: u32) -> Self {
    BArray {
      min: Some(value),
      ..self.clone()
    }
  }

  #[napi(ts_return_type = "BArray<R>")]
  pub fn max(&self, value: u32) -> Self {
    BArray {
      max: Some(value),
      ..self.clone()
    }
  }
}
