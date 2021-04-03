use worker::{FetchHandler, Error, WorkerRequest, WorkerResult};

#[derive(Default)]
pub struct Handler {}

impl FetchHandler<()> for Handler {
  fn handle(&self, _req: WorkerRequest) -> WorkerResult<()> {
    Err(Error::new("error response"))
  }
}