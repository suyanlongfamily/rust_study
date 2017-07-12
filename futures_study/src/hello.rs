#![feature(thread_id)]
//#![deny(warnings)]
extern crate hyper;
extern crate futures;
extern crate pretty_env_logger;

use futures::future::FutureResult;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

static PHRASE: &'static [u8] = b"Hello World!";

struct Hello;
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
impl Service for Hello {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;
    fn call(&self, _req: Request) -> Self::Future {
        let path = _req.uri().path();
        println!("-----path--{:?}-", path);
        println!(
            "---thread::current().id()---{:?}---",
            thread::current().id()
        );
        if path == "/" {
            std::thread::sleep(Duration::new(1, 100));
        } else {

        }
        println!("-----end---");


        futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(PHRASE),
        )
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Hello)).unwrap();
    println!(
        "Listening on http://{} with 1 thread.",
        server.local_addr().unwrap()
    );
    println!("---thread::current().id---{:?}---", thread::current().id());
    server.run().unwrap();
}
