//#![deny(warnings)]
#![allow(unused)]
extern crate futures;
extern crate hyper;
extern crate mta_status;
extern crate serde_json;
extern crate tokio_core;
extern crate futures_cpupool;

use futures_cpupool::CpuPool;
use tokio_core::reactor::{Core, Handle};
use hyper::Error;
use futures::future::FutureResult;
use hyper::server::{Server, Request, Response};
use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service};
use std::time::Duration;
use std::thread;
use futures::future::join_all;
use tokio_core::net::TcpListener;
use futures::{Future, BoxFuture, Stream, future};

#[cfg(debug_assertions)]
const IS_PROD: bool = false;

#[cfg(not(debug_assertions))]
const IS_PROD: bool = true;

struct GetStatus{
    _handle: Handle,
}

impl GetStatus {
    fn new(handle: Handle) -> Self {
        GetStatus{_handle: handle}
    }
}

impl Service for GetStatus {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = hyper::server::Response, Error = hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut resp = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let i = mta_status::get_status(&self._handle)
                    .map(|stat|
                        resp.with_body(stat).with_status(StatusCode::NotFound)
                    );
                Box::new(i)

            },
            _ => {
                Box::new(futures::future::ok(
                    resp.with_body("no path")
                    .with_status(StatusCode::NotFound),
                ))
            },
        }

    }
}

fn main() {
    println!("prod build: {}", IS_PROD);

    let mut core = Core::new().unwrap();
    let mut handle = core.handle();

    let addr = "127.0.0.1:4000".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle).unwrap();


    let http = Http::new();
    let server = listener.incoming().for_each(move |(sock, addr)| {
        let get_status_srv = GetStatus::new(handle.clone());
        http.bind_connection(&handle.clone(), sock, addr, get_status_srv);
        Ok(())
    });

    println!("http://localhost:4000");
    core.run(server).unwrap();
}
