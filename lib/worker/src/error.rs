use wasm_bindgen::prelude::*;

pub enum Error {
  ApplicationError(JsValue),
  NewHeaderFault(JsValue),
  UnableToAppendHeader(JsValue),
  InvalidHeader(JsValue)
}

impl Error {
  pub fn new(error: impl Into<JsValue>) -> Self {
    Error::ApplicationError(error.into())
  }
}

impl From<Error> for JsValue {
  fn from(src: Error) -> JsValue {
    match src {
      Error::ApplicationError(error) => error.into(),
      Error::NewHeaderFault(error) => error,
      Error::UnableToAppendHeader(error) => error,
      Error::InvalidHeader(error) => error,
    }
  }
}