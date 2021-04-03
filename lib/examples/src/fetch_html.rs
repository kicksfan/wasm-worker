use web_sys::{Request};

const someHost = "https://examples.cloudflareworkers.com/demos";
const url = someHost + "/static/html";

pub fn handler(_request: Request) -> Result<worker::Response, worker::Error> {
  let headers = 
}