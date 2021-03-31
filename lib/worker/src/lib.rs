use cfg_if::cfg_if;
use js_sys::{Promise};
use wasm_bindgen::prelude::*;
use web_sys::{FetchEvent, ResponseInit};
use web_sys::{Request};

cfg_if! {
  // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
  // allocator.
  if #[cfg(feature = "wee_alloc")] {
      extern crate wee_alloc;
      #[global_allocator]
      static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
  }
}

cfg_if! {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  if #[cfg(feature = "console_error_panic_hook")] {
    extern crate console_error_panic_hook;
    pub use self::console_error_panic_hook::set_once as set_panic_hook;
  } else {
    #[inline]
    pub fn set_panic_hook() {}
  }
}

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);

  // The `console.log` is quite polymorphic, so we can bind it with multiple
  // signatures. Note that we need to use `js_name` to ensure we always call
  // `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  pub fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  pub fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
extern "C" {
  pub fn fetch(req: &Request) -> Promise;
}

// As of writing, web-sys does not support creating Response objects, so
// we define our own wrapper here
#[wasm_bindgen]
extern "C" {

  pub type Response;

  #[wasm_bindgen(constructor)]
  pub fn new(body: &str, init: ResponseInit) -> Response;
}

// pub type FetchHandler<TError> = fn(FetchEvent) -> Result<Response, TError>;
pub type FetchHandler<TError> = fn(Request) -> Result<Response, TError>;

pub enum Error {
  NewHeaderInvalid(JsValue),
  UnableToAppendHeader(JsValue)
}

impl From<Error> for JsValue {
  fn from(src: Error) -> JsValue {
    match src {
      Error::NewHeaderInvalid(error) => error,
      Error::UnableToAppendHeader(error) => error,
    }
  }
}

pub struct Headers {
  inner: Vec<(String, String)>
}

impl Headers {
  pub fn new() -> Self {
    Headers {
      inner: vec![]
    }
  }

  pub fn from_tuples(values: Vec<(String, String)>) -> Self {
    Headers {
      inner: values
    }
  }

  fn into_web_sys_headers(&self) -> Result<web_sys::Headers, Error> {
    let result = web_sys::Headers::new().map_err(Error::NewHeaderInvalid)?;
    for (key, value) in &self.inner {
      result.append(&key, &value).map_err(Error::UnableToAppendHeader)?;
    }
    Ok(result)
  }
}

impl From<Vec<(String, String)>> for Headers {
  fn from(values: Vec<(String, String)>) -> Headers {
    Headers::from_tuples(values)
  }
}

// /**
//  * new_header builds an empty Header and maps failure into the worker Error enum
//  */
// pub fn new_header() -> Result<web_sys::Headers, Error> {
//   web_sys::Headers::new().map_err(Error::NewHeaderInvalid)
// }

// /**
//  * make_headers builds an empty Header and appends the supplied tuple into the values
//  */
// pub fn make_headers(values: Vec<(&str, &str)>) -> Result<web_sys::Headers, Error> {
//   let result = new_header()?;
//   for (key, value) in values {
//     result.append(key, value).map_err(Error::UnableToAppendHeader)?;
//   }
//   Ok(result)
// }

/**
 * make_response constructs a new ResponseInit, populates it and then maps it into
 * a new Response
 */
pub fn make_response<T: Into<Headers>>(body: &str, status: u16, headers: &T) -> Result<Response, Error> {
  let mut init = ResponseInit::new();
  init.status(status);
  let headers: Headers = headers.into();
  init.headers(&JsValue::from(&headers.into_web_sys_headers()?));
  Ok(Response::new(body, init))
}

/**
 * js_handler is a thin helper which maps a FetchEvent across the provided handler
 * and handles marshaling into JsValues
 */
pub fn js_handler<TError: Into<JsValue>>(handler: FetchHandler<TError>) -> impl Fn(FetchEvent) -> Promise {
  console_error_panic_hook::set_once();
  move | event: FetchEvent | {
    log(&format!("client id: {:?}", event.client_id()));
    let request = event.request();
    log(&format!("request: {:?}", request));
    // TODO: Change handler to take the Request instead of the root FetchEvent struct...
    match handler(request) {
      Ok(success) => Promise::resolve(&JsValue::from(success)),
      Err(error) => Promise::reject(&error.into())
    }
  }
}