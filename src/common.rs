use napi::{
  bindgen_prelude::ToNapiValue, sys::napi_env__, sys::napi_value__, Env, Error, JsUnknown,
  NapiValue, Result, Status,
};

use crate::{BType, BValueBase};

pub struct ClonableJsUnknown {
  napi_env: *mut napi_env__,
  napi_value: *mut napi_value__,
}

impl ClonableJsUnknown {
  pub fn new(env: Env, value: JsUnknown) -> Result<Self> {
    let napi_env = env.raw();
    let napi_value = unsafe { JsUnknown::to_napi_value(napi_env, value)? };
    Ok(ClonableJsUnknown {
      napi_env,
      napi_value,
    })
  }

  pub fn create_clone(&self) -> Result<JsUnknown> {
    Ok(unsafe { JsUnknown::from_raw(self.napi_env, self.napi_value)? })
  }
}

pub fn parse_btype(b_type: &BType, value: JsUnknown, env: Env) -> Result<JsUnknown> {
  match &*b_type {
    BType::Undefined(s) => s.parse(value),
    BType::Null(s) => s.parse(value),
    BType::Boolean(s) => s.parse(value),
    BType::Number(s) => s.parse(value),
    BType::String(s) => s.parse(value),
    BType::Array(s) => s.parse(env, value),
    BType::Object(s) => s.parse(env, value),
    BType::Union(s) => s.parse(env, value),
    BType::Tuple(s) => s.parse(env, value),
  }
}

// pub fn parse_btype_rc(b_type: &BType, value: Rc<JsUnknown>, env: Env) -> Result<JsUnknown> {
//   match Rc::try_unwrap(value) {
//     Ok(v) => parse_btype(b_type, v, env),
//     Err(rc) => Err(Error::new(
//       Status::GenericFailure,
//       format!(
//         "Failed to unwrap, Rc<JsUnkown> still has references, value_type: {:?}, weak_count: {}, strong_count: {}",
//         rc.get_type()?, Rc::weak_count(&rc), Rc::strong_count(&rc)
//       ),
//     )),
//   }
// }

pub fn get_btype_base(b_type: &BType) -> &BValueBase {
  match b_type {
    BType::Undefined(s) => s.get_base(),
    BType::Null(s) => s.get_base(),
    BType::Boolean(s) => s.get_base(),
    BType::Number(s) => s.get_base(),
    BType::String(s) => s.get_base(),
    BType::Array(s) => s.get_base(),
    BType::Object(s) => s.get_base(),
    BType::Union(s) => s.get_base(),
    BType::Tuple(s) => s.get_base(),
  }
}

pub fn invalid_arg(reason: &str) -> Error {
  Error::new(Status::InvalidArg, reason)
}
