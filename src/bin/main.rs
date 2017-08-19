#![deny(warnings)]
//#![allow(unused)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate futures;
extern crate hyper;
extern crate mta_status_lib;
extern crate serde_json;
extern crate tokio_core;

use tokio_core::reactor::{Core, Handle};
use hyper::server::{Request, Response};
use hyper::{Method, StatusCode};
use hyper::server::{Http, Service};
use tokio_core::net::TcpListener;
use futures::{Future, Stream};

#[cfg(debug_assertions)]
const IS_PROD: bool = false;

#[cfg(not(debug_assertions))]
const IS_PROD: bool = true;

struct GetStatus {
    _handle: Handle,
}

impl GetStatus {
    fn new(handle: Handle) -> Self {
        GetStatus { _handle: handle }
    }
}

impl Service for GetStatus {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let resp = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let status = mta_status_lib::get_status(&self._handle).map(|stat| {
                    resp.with_body(stat).with_status(StatusCode::NotFound)
                });
                Box::new(status)
            },
            _ => Box::new(futures::future::ok(
                resp.with_body("no path").with_status(StatusCode::NotFound),
            )),
        }

    }
}

fn main() {
    println!("prod build: {}", IS_PROD);

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "127.0.0.1:4000".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle).unwrap();

    let http = Http::new();
    let server = listener.incoming().for_each(|(sock, addr)| {
        let get_status_srv = GetStatus::new(handle.clone());
        http.bind_connection(&handle, sock, addr, get_status_srv);
        Ok(())
    });

    println!("http://localhost:4000");
    core.run(server).unwrap();
}
