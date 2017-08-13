//#![deny(warnings)]
extern crate hyper;
extern crate futures;
extern crate tokio_core;

use std::str;
use std::io::Read;
use hyper::client::HttpConnector;
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::{Core, Handle};
use hyper::client::Response;
use futures::future::BoxFuture;
use hyper::client::FutureResponse;
use futures::future::Map;

pub fn get_mta_status(handle: &Handle) -> Box<Future<Item=String, Error=hyper::Error>> {
    // This is not a txt file, but actually a URL which returns a XML response with HTML
    // embedded inside.. yuk. The purpose of this project is to take that XML/HTML
    // response and convert it into a nice JSON response :)
    let uri: hyper::Uri = "http://web.mta.info/status/serviceStatus.txt".parse().unwrap();

    let fut_resp = Client::new(&handle).get(uri)
        //todo check if this succeeds with `then`
        .and_then(|resp| {

        let str_body = resp.body().concat().map(move |chunk_body: hyper::Chunk| {
            match str::from_utf8(&chunk_body) {
                Ok(v) => v.to_string(),
                Err(e) => "{}".to_string(),
            }
        });

        str_body
    });
    Box::new(fut_resp)
    //Box::new(futures::future::ok("doing".to_string()))
}
