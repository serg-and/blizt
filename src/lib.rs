use common::invalid_arg;
use napi::{JsUnknown, Result, ValueType};

mod array;
mod boolean;
mod common;
mod null;
mod number;
mod object;
mod string;
mod tuple;
mod undefined;
mod union;

#[macro_use]
extern crate napi_derive;

#[derive(Debug, Clone, PartialEq)]
pub enum BType {
  Undefined(undefined::BUndefined),
  Null(null::BNull),
  Boolean(boolean::BBoolean),
  Number(number::BNumber),
  String(string::BString),
  Array(array::BArray),
  Object(object::BObject),
  // Map,
  // Set,
  // Date,
  // Function,
  // Buffer,
  // Arraybuffer,
  Union(union::BUnion),
  Tuple(tuple::BTuple),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BValueBase {
  optional: bool,
  nullable: bool,
}

impl BValueBase {
  pub fn default() -> BValueBase {
    BValueBase {
      optional: false,
      nullable: false,
    }
  }

  pub fn skip_parse(&self, value: &JsUnknown) -> Result<bool> {
    let value_type = value.get_type()?;

    if value_type == ValueType::Undefined {
      return match self.optional {
        true => Ok(true),
        false => Err(invalid_arg("Got undefined for required value")),
      };
    }
    if value_type == ValueType::Null {
      return match self.nullable {
        true => Ok(true),
        false => Err(invalid_arg("Got Null for non nullable value")),
      };
    }

    Ok(false)
  }
}

#[napi]
#[derive(Debug, Clone, PartialEq)]
pub struct BWrapped {
  inner: BType,
}

#[napi(object)]
pub struct ParseSafeRes {
  pub success: bool,
  pub data: Option<JsUnknown>,
  pub reason: Option<String>,
}

#[macro_export]
macro_rules! impl_parse_safe_method {
  ($S:ident) => {
    use crate::ParseSafeRes;

    #[napi]
    impl $S {
      #[napi(ts_return_type = "ParseSafe<R>")]
      pub fn parse_safe(&self, value: JsUnknown) -> ParseSafeRes {
        match self.parse(value) {
          Ok(data) => ParseSafeRes {
            success: true,
            data: Some(data),
            reason: None,
          },
          Err(error) => ParseSafeRes {
            success: false,
            data: None,
            reason: Some(error.reason),
          },
        }
      }
    }
  };
}

#[macro_export]
macro_rules! impl_parse_safe_method_with_env {
  ($S:ident) => {
    use crate::ParseSafeRes;

    #[napi]
    impl $S {
      #[napi(ts_return_type = "ParseSafe<R>")]
      pub fn parse_safe(&self, env: Env, value: JsUnknown) -> ParseSafeRes {
        match self.parse(env, value) {
          Ok(data) => ParseSafeRes {
            success: true,
            data: Some(data),
            reason: None,
          },
          Err(error) => ParseSafeRes {
            success: false,
            data: None,
            reason: Some(error.reason),
          },
        }
      }
    }
  };
}

#[macro_export]
macro_rules! impl_base_methods {
  ($S:ident, $b_type:ident, $optional:expr, $nullable:expr, $nullish:expr, $required:expr, $non_nullable:expr) => {
    #[napi]
    impl $S {
      // #[napi(constructor)]
      // pub fn new() -> $S {
      //   $S::default()
      // }

      pub fn get_base(&self) -> &BValueBase {
        &self.base
      }

      #[napi(ts_return_type = $optional)]
      pub fn optional(&self) -> Self {
        let mut c = self.clone();
        c.base.optional = true;
        c
      }

      #[napi(ts_return_type = $nullable)]
      pub fn nullable(&self) -> Self {
        let mut c = self.clone();
        c.base.nullable = true;
        c
      }

      #[napi(ts_return_type = $nullish)]
      pub fn nullish(&self) -> Self {
        let mut c = self.clone();
        c.base.optional = true;
        c.base.nullable = true;
        c
      }

      #[napi(ts_return_type = $required)]
      pub fn required(&self) -> Self {
        let mut c = self.clone();
        c.base.optional = false;
        c
      }

      #[napi(ts_return_type = $non_nullable)]
      pub fn non_nullable(&self) -> Self {
        let mut c = self.clone();
        c.base.nullable = false;
        c
      }

      /// For internal use only!
      /// Create BWrapped instance of this value to be used by other methods
      #[napi(js_name = "_toWrapped")]
      pub fn to_wrapped(&self) -> crate::BWrapped {
        crate::BWrapped {
          inner: crate::BType::$b_type(self.to_owned()),
        }
      }
    }
  };
}
