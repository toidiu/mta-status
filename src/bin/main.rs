//#![deny(warnings)]
#![allow(unused)]
extern crate futures;
extern crate hyper;
extern crate mta_status;
extern crate serde_json;
extern crate tokio_core;
extern crate futures_cpupool;

//use futures::Future;
use futures_cpupool::CpuPool;
use tokio_core::reactor::{Core, Handle};
use hyper::Error;
use futures::future::FutureResult;
//use futures::future::BoxFuture;

use hyper::server::{Server, Request, Response};
use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service};
use std::time::Duration;
use std::thread;
use futures::future::join_all;
use tokio_core::net::TcpListener;
use futures::{Future, BoxFuture, Stream, future};



struct GetStatus<'a>{
    _handle: &'a Handle,
}

impl<'a> GetStatus<'a> {
    fn new(handle: &'a Handle) -> Self {
        GetStatus{_handle: handle}
    }
}

impl<'a> Service for GetStatus<'a> {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = hyper::server::Response, Error = hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut resp = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
//                    let mta = mta_status::get_status();
//                    futures::future::ok(
//                       resp.with_body(mta)
//                    )

                let i = mta_status::get_status()
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
//    let addr = "127.0.0.1:4000".parse().unwrap();
//    let server = Http::new().bind(&addr, || Ok(GetStatus)).unwrap();
//    server.run().unwrap();



    let mut core = Core::new().unwrap();
    let mut handle = core.handle();

    let addr = "127.0.0.1:4000".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle).unwrap();

    let &mut  han =  handle.clone();

    let http = Http::new();
    let server = listener.incoming().for_each(move |(sock, addr)| {
        let s = GetStatus::new(han);
//        let s = MainService::new(&handle);
//        let s = GetStatus{_handle: &handle};

        http.bind_connection(&handle, sock, addr, s);
        Ok(())
    });

    println!("http://localhost:4000");
    core.run(server).unwrap();




}
