//#![deny(warnings)]
//#![allow(unused)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
// #![feature(use_extern_macros)]

extern crate futures;
//extern crate hyper;
#[macro_use] extern crate hyper;
extern crate mta_status;
extern crate tokio_core;
#[macro_use] extern crate log;
extern crate log4rs;

use std::fs::File;
use std::io::Write;

use tokio_core::reactor::{Core, Handle};
use hyper::server::{Request, Response};
use hyper::{Method, StatusCode};
use hyper::server::{Http, Service};
use tokio_core::net::TcpListener;
use futures::{Future, Stream};

use hyper::header::Headers;
header! { (AccessControl, "Access-Control-Allow-Origin") => [String] }

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
    type Future = Box<Future<Item = hyper::server::Response, Error = hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let resp = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let status = mta_status::get_status(&self._handle).map(|stat| {
                    let mut headers = Headers::new();
                    headers.set(AccessControl("*".to_owned()));

                    resp
                        .with_body(stat)
                        .with_headers(headers)
                        .with_status(StatusCode::Ok)
                });
                Box::new(status)
            }
            _ => Box::new(futures::future::ok(
                resp
                .with_body("no path")
                .with_status(StatusCode::NotFound),
            )),
        }

    }
}

fn main() {
    let data = include_str!("../../resources/log4rs.yaml");
    let mut f = File::create("log_config.yaml").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    info!("prod build: {}", IS_PROD);

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "127.0.0.1:4000".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle).unwrap();


    let server = listener.incoming().for_each(move |(sock, addr)| {
        let get_status_srv = GetStatus::new(handle.clone());
        Http::new().bind_connection(&handle.clone(), sock, addr, get_status_srv);
        Ok(())
    });

    info!("http://localhost:4000");
    core.run(server).unwrap();
}
