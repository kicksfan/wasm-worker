// use http::{HeaderMap, StatusCode};
use js_sys::{Promise};
use wasm_bindgen::prelude::*;
use web_sys::{FetchEvent, ResponseInit};

// use web_sys::{Request};

pub mod console;
mod error;
mod init;
pub mod ffi;

pub use error::*;

// pub type Result = std::result::Result<dyn WorkerResponse, Error>;

// // wasm_bindgen::JsValue: From<std::result::Result<(dyn WorkerResponse + 'static), error::Error>>

// impl From<Result> for JsValue {
//   fn from(src: Result) -> JsValue {
//     JsValue::from_str("temp")
//   }
// }

// impl Into<JsValue> for Result {
//   fn into(src: Result) -> JsValue {
//     match src {
//       Ok(ref success) => success.into(),
//       Err(err) => err.into()
//     }
//   }
// }

pub struct WorkerResponse<T> {
  _inner: http::Response<T>
}

impl<T> WorkerResponse<T> {
  pub fn new(body: T) -> Self {
    WorkerResponse {
      _inner: http::Response::new(body)
    }
  }
}

impl<T> From<T> for WorkerResponse<T> {
  fn from(src: T) -> WorkerResponse<T> {
    WorkerResponse::new(src)
  }
}

// impl<T> Into<Result<WorkerResponse<T>, Error>> for T {
//   fn into(self) -> Result<WorkerResponse<T>, Error> {
//     Ok(self.into())
//   }
// }

// impl<T> From<T> for Result<WorkerResponse<T>, Error> {
//   fn from(src: T) -> WorkerResult<T> {
//     Ok(src.into())
//   }
// }

impl<T> Into<JsValue> for WorkerResponse<T> {
  fn into(self) -> JsValue {
    let mut init = ResponseInit::new();
    init.status(200);
    let resp = ffi::Response::new("foo bar for life", init);
    JsValue::from(resp)
  }
}

pub type WorkerResult<T> = Result<WorkerResponse<T>, Error>;
// pub type WorkerResult = dyn WorkerResponse;

pub enum Html {
  Empty
}

impl Into<WorkerResponse<&str>> for Html {
  fn into(self) -> WorkerResponse<&'static str> {
    match self {
      Html::Empty => WorkerResponse::new("worker response mapping")
    }
  }
}

// impl<T: WorkerResponse> Into<T> for Html {
//   fn into(src: Html) -> T {
//     match src {
//       Html::Empty => "worker response mapping"
//     }
//   }
// }

// pub type FetchHandler<TError> = fn(FetchEvent) -> Result<Response, TError>;
// pub type FetchHandler<TError> = fn(Request) -> Result<Response, TError>;
// pub type FetchHandler = fn(WorkerRequest) -> Result;
// pub type FetchHandler<T: Into<dyn WorkerResponse>> = fn(WorkerRequest) -> T;
// pub type FetchHandler<T, R: Into<WorkerResponse<T>>>= fn(WorkerRequest) -> R;

// pub struct WorkerResponse<T, R: Into<WorkerResponder<T>>> {
//   responder: R
// }

pub trait FetchHandler<T> {
  fn handle(&self, req: WorkerRequest) -> WorkerResult<T>;
}

// pub enum StatusCodes {
//   Ok
// }

pub struct WorkerRequest {
  pub(crate) inner: FetchEvent
}

impl WorkerRequest {
  pub fn from_fetch_event(event: FetchEvent) -> Self {
    WorkerRequest {
      inner: event
    }
  }
}



// pub enum WorkerResponse {
//   Empty 
//   // Html { body: &str, status: Status, headers: Option<Headears> }
// }

// impl<T> Into<JsValue> for T
// where
//   T: WorkerResponse
// {
//   fn into(_rsp: T) -> JsValue {
//     JsValue::from_str("blamo")
//   }
// }

// impl<T> From<T> for JsValue
// where
//   T: WorkerResponse
// {
//   fn from(_request: WorkerResponse) -> JsValue {
//     JsValue::from_str("blamo")
//   }
// }

/* convert to web_sys::Response
impl AsRef<web_sys::Response> for WorkerResponse {
  fn as_ref(&self) -> &web_sys::Response {
    &self.into_web_sys_response()
  }
}

impl AsMut<web_sys::Response> for WorkerResponse {
  fn as_mut(&mut self) -> &mut web_sys::Response {
    &mut self.into_web_sys_response()
  }
}
*/

// Make response a builder pattern?  Look at other html libs

// Need to integrate with CF new workers stateful model

/**
 * make_response constructs a new ResponseInit, populates it and then maps it into
 * a new Response
 */
pub fn make_response<T>(body: T, status: u16, headers: Vec<(String, String)>) -> Html {
  let mut init = ResponseInit::new();
  init.status(status);
  // init.headers(&JsValue::from(&headers.into().into_web_sys_headers()?));
  // Ok(WorkerResponse::new(body, init))
  Html::Empty
}

/**
 * js_handler is a thin helper which maps a FetchEvent across the provided handler
 * and handles marshaling into JsValues
 */
pub fn js_handler<T>(handler: impl FetchHandler<T>) -> impl Fn(FetchEvent) -> Promise {
  console_error_panic_hook::set_once();
  move | event: FetchEvent | {
    console::log(&format!("client id: {:?}", event.client_id()));
    // let request = event.request();
    let request = WorkerRequest::from_fetch_event(event);
    console::log(&format!("request: {:?}", request.inner.request()));
    let result = handler.handle(request);
    // TODO: Change handler to take the Request instead of the root FetchEvent struct...
    // let result = response.into();
    match result {
      // Ok(success) => Promise::resolve(&JsValue::from(success)),
      Ok(response) => Promise::resolve(&response.into()),
      Err(error) => Promise::reject(&error.into())
    }
  }
}


// impl<S, I, T> From<T> for Headers<S, I>
// where
//   S: Into<String>,
//   I: Iterator<Item = (S, S)>,
//   T: Into<I>
// {
//   fn from(values: T) -> Headers<S, I> {
//     Headers::new(values)
//   }
// }

// impl From<Vec<(&str, &str)>> for Headers {
//   fn from(values: Vec<(&str, &str)>) -> Headers {
//     let result = vec![];
//     for (key, value) in &values {
//       result.append((key.to_owned(), value.to_owned()))
//     }
//     let values = values.iter().map(|&(ref l, ref r)| -> (String, String) {
//       (l.to_owned(), r.to_owned())
//     });
//     Headers::from_tuples(values)
//   }
// }

// /**
//  * new_header builds an empty Header and maps failure into the worker Error enum
//  */
// pub fn new_header() -> Result<web_sys::Headers, Error> {
//   web_sys::Headers::new().map_err(Error::NewHeaderFault)
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



// pub struct Headers {
//   inner: Option<Vec<(String, String)>>
// }

// impl Headers {
//   pub fn empty() -> Self {
//     Headers {
//       inner: None
//     }
//   }

//   pub fn new(values: Vec<(String, String)>) -> Result<Self, Error> {
//     Headers {
//       inner: Some(values)
//     }
//   }
// }

// pub struct Headers<S, I>
// where
//   S: Into<String>,
//   I: Clone + IntoIterator<Item = (S, S)>
// {
//   inner: Option<I>
// }

// impl<S, I> Headers<S, I>
// where
//   S: Into<String>,
//   I: Clone + IntoIterator<Item = (S, S)>
// {
//   pub fn empty() -> Self {
//     Headers {
//       inner: None
//     }
//   }

//   pub fn new(values: I) -> Self {
//     Headers {
//       inner: Some(values)
//     }
//   }

//   fn into_web_sys_headers(&mut self) -> std::result::Result<web_sys::Headers, Error> {
//     let result = web_sys::Headers::new().map_err(Error::NewHeaderFault)?;
//     if let Some(ref mut values) = self.inner {
//       for (key, value) in values.to_owned().into_iter() {
//         result.append(&key.into(), &value.into()).map_err(Error::UnableToAppendHeader)?;
//       }
//     }
//     Ok(result)
//   }
// }

// impl<S, I> From<Vec<(&str, &str)>> for Headers<S, I>
// where
//   S: Into<String>,
//   I: Clone + IntoIterator<Item = (S, S)>
// {
//   fn from(_values: Vec<(&str, &str)>) -> Headers<S, I> {
//     // let v: Vec<(S, S)> = values.iter().map(|&(k, v)| (k.to_owned(), v.to_owned())).collect::<(S, S)>();
//     Headers::empty()//new(values)
//   }
// }

// pub struct Response {
//   body: &str,
//   status: u16,
//   headers: Option<web_sys::Headers>,
// }
