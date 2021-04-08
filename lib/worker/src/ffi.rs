use wasm_bindgen::prelude::*;
use js_sys::{Promise};
use web_sys::{Request, ResponseInit};

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
  pub fn new(body: &[u8], init: ResponseInit) -> Response;
}