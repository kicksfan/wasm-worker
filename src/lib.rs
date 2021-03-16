extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;
mod worker;

use cfg_if::cfg_if;
use js_sys::{Promise};
use wasm_bindgen::prelude::*;
use web_sys::{FetchEvent, Headers, Request};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

enum Error {
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

type FetchHandler<TError> = fn(FetchEvent) -> Result<worker::Response, TError>;

const HANDLER: FetchHandler<Error> = example_static_html;

#[wasm_bindgen]
pub fn js_handler(event: FetchEvent) -> Promise {
    console_error_panic_hook::set_once();
    match HANDLER(event) {
      Ok(success) => Promise::resolve(&JsValue::from(success)),
      Err(error) => Promise::reject(&JsValue::from(error))
    }
    // let response = handler(event);
    // worker::log("foo");
    // Promise::resolve(&JsValue::from(response))
}

fn example_static_html(_event: FetchEvent) -> Result<worker::Response, Error> {
  // let req = &event.request();
  let body = "<html><head>WASM</head><body>WASM Generated</body></html>";
  let headers = Headers::new().map_err(Error::NewHeaderInvalid)?;
  headers.append("content-type", "text/html").map_err(Error::UnableToAppendHeader)?;
  Ok(worker::make_response(&body, 404, &headers))
}

fn example_error_response(_event: FetchEvent) -> Result<worker::Response, Error> {
  Err(Error::NewHeaderInvalid(JsValue::from("foo")))
}

// fn wasm_generated(_event: FetchEvent) -> Result<worker::Response, Error> {
//   let body = "<html><head>WASM</head><body>WASM Generated</body></html>";
//   let headers = Headers::new().unwrap();//?;
//   headers.append("content-type", "text/html").unwrap();//?;
//   worker::make_response(&body, 200, &headers)
// }



// #[wasm_bindgen]
// pub fn greet() -> String {
//     "Hello, wasm-worker!".to_string()
// }

// #[wasm_bindgen]
// pub fn parse() -> String {
//     let markdown_input: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";
//     println!("Parsing the following Markdown string:\n{}", markdown_input);

//     // Set up options and parser. Strikethroughs are not part of the CommonMark standard
//     // and we therefore must enable it explicitly.
//     let mut options = Options::empty();
//     options.insert(Options::ENABLE_STRIKETHROUGH);
//     let parser = Parser::new_ext(markdown_input, options);

//     // Write to String buffer.
//     let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
//     html::push_html(&mut html_output, parser);

//     // Check that the output is what we expected.
//     let expected_html: &str = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
//     assert_eq!(expected_html, &html_output);

//     format!("\nHTML output:\n{}", &html_output)
// }