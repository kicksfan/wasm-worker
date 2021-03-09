extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use js_sys::{Promise};
use wasm_bindgen::prelude::*;
use web_sys::{FetchEvent, Headers, Request, ResponseInit};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// As of writing, web-sys does not support creating Response objects, so
// we define our own wrapper here
#[wasm_bindgen]
extern "C" {
    type Response;

    #[wasm_bindgen(constructor)]
    fn new(body: &str, init: ResponseInit) -> Response;
}

#[wasm_bindgen]
extern "C" {
    // fn addEventListener(s: &str, f: Function);
    // fn addEventListener(s: &str, f: &dyn Fn(FetchEvent) -> Promise);

    fn fetch(req: &Request) -> Promise;
}

#[wasm_bindgen]
pub fn handle_cloudflare_fetch(event: FetchEvent) -> Promise {
    // console_error_panic_hook::set_once();
    let response = handler(event);
    log("foo");
    Promise::resolve(&JsValue::from(response))
}

fn handler(_event: FetchEvent) -> Response {
  // let req = &event.request();
  let body = "<html><head>WASM</head><body>WASM Generated</body></html>";
  let headers = Headers::new().unwrap();//?;
  headers.append("content-type", "text/html").unwrap();//?;
  generate_response(&body, 404, &headers)
}

fn generate_response(body: &str, status: u16, headers: &Headers) -> Response {
  let mut init = ResponseInit::new();
  init.status(status);
  init.headers(&JsValue::from(headers));
  Response::new(body, init)
}

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