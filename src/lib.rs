extern crate rocket;
extern crate futures;
extern crate tokio;

use rocket::response::Responder;
use rocket::{ response, Request };
use futures::Future;
use tokio::runtime::{ self, current_thread };

use std::fmt::Debug;

pub struct Wrapper<F>(F);

impl<F> Wrapper<F> {
    pub fn new(value: F) -> Self {
        Wrapper(value)
    }
}

impl<F> Into<Runtime<F>> for Wrapper<F> {
    fn into(self) -> Runtime<F> {
        Runtime(self.0)
    }
}

impl<F> Into<Current<F>> for Wrapper<F> {
    fn into(self) -> Current<F> {
        Current(self.0)
    }
}

pub struct Runtime<F>(F);

pub struct Current<F>(F);

impl<'r, F> Responder<'r> for Runtime<F>
    where F: Future + Send + 'static,
          F::Item: Send + Responder<'r> + 'static,
          F::Error: Send + Debug + 'static {

    fn respond_to(self, req: &Request) -> response::Result<'r> {
        runtime::Runtime::new()
            .unwrap()
            .block_on(self.0)
            .respond_to(req)
    }
}

impl<'r, F> Responder<'r> for Current<F>
    where F: Future,
        F::Item: Responder<'r>,
        F::Error: Debug {

    fn respond_to(self, req: &rocket::Request) -> response::Result<'r> {
        current_thread::Runtime::new()
            .unwrap()
            .block_on(self.0)
            .respond_to(req)
    }
}

