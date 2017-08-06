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

    struct GetStatus;

    impl Service for GetStatus {
        type Request = hyper::server::Request;
        type Response = hyper::server::Response;
        type Error = hyper::Error;
        type Future = Box<Future<Item = Self::Response, Error = Self::Error> + std::marker::Send>;

        fn call(&self, req: Request) -> Self::Future {
            let mut resp = Response::new();

            match (req.method(), req.path()) {
                (&Method::Get, "/") => {
                    futures::future::ok(
                       resp.with_body("hi ya".to_string())
                    ).boxed()
                },
                _ => {
                    futures::future::ok(
                        resp.with_body("no path".to_string())
                        .with_status(StatusCode::NotFound),
                    ).boxed()
                },
            }

        }
    }

    println!("http://localhost:4000");
    let addr = "127.0.0.1:4000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(GetStatus)).unwrap();
    server.run().unwrap();
}
