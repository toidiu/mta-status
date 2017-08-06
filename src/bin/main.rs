//#![deny(warnings)]
#![allow(unused)]
extern crate futures;
extern crate hyper;
extern crate mta_status;
extern crate serde_json;
extern crate tokio_core;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;
use tokio_core::reactor::Core;
use futures::BoxFuture;
use hyper::Error;
use futures::future::FutureResult;

use hyper::server::{Server, Request, Response};
use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service};
use std::time::Duration;
use std::thread;

use futures::future::join_all;

fn main() {
    //    const PHRASE: &'static str = "Hello, World!";


    let pool = CpuPool::new_num_cpus();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let fut = pool.spawn_fn(|| {
        let prime = true;

        thread::sleep(Duration::from_secs(5));
        println!("hi from the future");


        let res: Result<bool, ()> = Ok(prime);
        res
    });

    let fut1 = pool.spawn_fn(|| {
        let prime = true;

        thread::sleep(Duration::from_secs(2));
        println!("hi from the future2");


        let res: Result<bool, ()> = Ok(prime);
        res
    });

    let h = handle.spawn_fn(|| {
        println!("hi from the future1");
        //        let prime = true;
        //        let res: Result<bool, bool> = Ok(prime);
        //        res
        Ok(())
    });


    let ff= fut.join(fut1);

    core.run(ff);


    //    struct HelloWorld;
    //    impl Service for HelloWorld {
    //        // boilerplate hooking up hyper's server types
    //        type Request = Request;
    //        type Response = Response;
    //        type Error = hyper::Error;
    //        // The future representing the eventual Response your call will
    //        // resolve to. This can change to whatever Future you need.
    //        type Future = futures::future::FutureResult<Self::Response, Self::Error>;
    //
    //        fn call(&self, _req: Request) -> Self::Future {
    //            thread::sleep(Duration::from_secs(5));
    //
    //            futures::future::ok(
    //                Response::new()
    //                    //.with_header(ContentLength(PHRASE.len() as u64))
    //                    //.with_body(PHRASE)
    //                    .with_header(ContentLength(prime.to_string().len() as u64))
    //                    .with_body(prime.to_string())
    //            )
    //        }
    //    }
    //
    //
    //    struct Echo;
    //    impl Service for Echo {
    //    type Future = Either<
    //        FutureResult<Self::Response, Self::Error>,
    //        Map<Concat2<Body>, fn(Chunk) -> Self::Response>
    //    >;
    //    // back to default Response
    //        type Response = Response;
    //
    //        fn call(&self, req: Request) -> Self::Future {
    //             match (req.method(), req.path()) {
    //                (&Method::Get, "/") => {
    //                    Either::A(futures::future::ok(
    //                        Response::new().with_body("Try POSTing data to /echo")
    //                    ))
    //                },
    //                (&Method::Post, "/echo") => {
    //                    Either::B(
    //                        req.body()
    //                            .concat2()
    //                            .map(reverse)
    //                    )
    //                },
    //                _ => {
    //                    Either::A(futures::future::ok(
    //                        Response::new().with_status(StatusCode::NotFound)
    //                    ))
    //                },
    //            }
    //        }
    //    }


    println!("http://localhost:4000");
    //    let addr = "127.0.0.1:4000".parse().unwrap();
    //    let server = Http::new().bind(&addr, || Ok()).unwrap();
    //    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    //    server.run().unwrap();
}
