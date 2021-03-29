// use web_sys::{FetchEvent};
use web_sys::{Request};

pub fn static_html(_request: Request) -> Result<worker::Response, worker::Error> {
  let body = "<html><head>WASM</head><body>WASM Generated</body></html>";
  let headers = worker::make_headers(vec![("content-type", "text/html")])?;
  Ok(worker::make_response(&body, 200, &headers))
}

pub fn error_response(_request: Request) -> Result<worker::Response, worker::Error> {
  Err(worker::Error::NewHeaderInvalid("foo".into()))
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