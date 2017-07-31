//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate mta_status;
extern crate serde_json;

use hyper::server::{Server, Request, Response};
use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service};
use std::time::Duration;
use std::thread;
use futures::future::{Either, Map};
use futures::stream::Concat2;
use std::ascii::AsciiExt;
use futures::Stream;
use futures::stream::Map;
use hyper::Chunk;

fn main() {


    const PHRASE: &'static str = "Hello, World!";

    struct HelloWorld;
    impl Service for HelloWorld {
        // boilerplate hooking up hyper's server types
        type Request = Request;
        type Response = Response;
        type Error = hyper::Error;
        // The future representing the eventual Response your call will
        // resolve to. This can change to whatever Future you need.
        type Future = futures::future::FutureResult<Self::Response, Self::Error>;

        fn call(&self, _req: Request) -> Self::Future {
            thread::sleep(Duration::from_secs(5));

            futures::future::ok(
                Response::new()
                    //.with_header(ContentLength(PHRASE.len() as u64))
                    //.with_body(PHRASE)
                    .with_header(ContentLength(prime.to_string().len() as u64))
                    .with_body(prime.to_string())
            )
        }
    }


    struct Echo;
    impl Service for Echo {
    type Future = Either<
        FutureResult<Self::Response, Self::Error>,
        Map<Concat2<Body>, fn(Chunk) -> Self::Response>
    >;
    // back to default Response
        type Response = Response;

        fn call(&self, req: Request) -> Self::Future {
             match (req.method(), req.path()) {
                (&Method::Get, "/") => {
                    Either::A(futures::future::ok(
                        Response::new().with_body("Try POSTing data to /echo")
                    ))
                },
                (&Method::Post, "/echo") => {
                    Either::B(
                        req.body()
                            .concat2()
                            .map(reverse)
                    )
                },
                _ => {
                    Either::A(futures::future::ok(
                        Response::new().with_status(StatusCode::NotFound)
                    ))
                },
            }
        }
    }



    println!("http://localhost:4000");
    let addr = "127.0.0.1:4000".parse().unwrap();
    //let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();

}
