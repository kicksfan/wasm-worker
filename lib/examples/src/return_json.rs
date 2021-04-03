use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Request};

#[derive(Serialize, Deserialize)]
struct ExampleJson<'a> {
  hello: &'a str
}

pub enum ReturnJsonError {
  WorkerError(worker::Error),
  SerdeError(serde_json::Error)
}

impl From<worker::Error> for ReturnJsonError {
  fn from(error: worker::Error) -> ReturnJsonError {
    ReturnJsonError::WorkerError(error)
  }
}

impl From<serde_json::Error> for ReturnJsonError {
  fn from(error: serde_json::Error) -> ReturnJsonError {
    ReturnJsonError::SerdeError(error)
  }
}

impl From<ReturnJsonError> for JsValue {
  fn from(src: ReturnJsonError) -> JsValue {
    match src {
      ReturnJsonError::WorkerError(error) => error.into(),
      ReturnJsonError::SerdeError(error) => JsValue::from(error.to_string())
    }
  }
}

pub fn handler(_request: Request) -> Result<worker::Response, ReturnJsonError> {
  let data = ExampleJson {
    hello: "world"
  };
  let body = serde_json::to_string(&data)?;
  // let headers = worker::Headers::from_tuples(vec![("content-type", "application/json;charset=UTF-8")])?;
  // let headers = worker::make_headers()?;
  // worker::make_response(&body, 200, &headers)
  worker::make_response(&body, 200, worker::Headers::empty())
}