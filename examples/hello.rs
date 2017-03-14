#![deny(warnings)]
extern crate hyper;
extern crate futures;
extern crate pretty_env_logger;
//extern crate num_cpus;
extern crate tokio_core;

use futures::future::FutureResult;

use tokio_core::reactor::Core;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

static PHRASE: &'static [u8] = b"Hello World!";

#[derive(Clone, Copy)]
struct Hello;

impl Service for Hello {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;
    fn call(&self, _req: Request) -> Self::Future {
        futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(PHRASE)
        )
    }

}

fn main() {
    pretty_env_logger::init().unwrap();
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, &core.handle(), || Ok(Hello)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run(&mut core).unwrap();
}
