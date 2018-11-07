extern crate rocket;
extern crate futures;
extern crate tokio;

use rocket::response::{ Responder, Response };
use rocket::http::Status;
use rocket::Request;
use futures::Future;
use tokio::runtime::Runtime;
use std::fmt::Debug;

pub struct Wrapper<T>(T);

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper(value)
    }
}

impl<'r, T> Responder<'r> for Wrapper<T>
    where T: Future + Send + 'static,
          T::Item: Send + Responder<'r> + 'static,
          T::Error: Send + Debug + 'static {

    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        Runtime::new().unwrap().block_on(self.0).respond_to(req)
    }
}
