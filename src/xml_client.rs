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

    let client = Client::new(&handle);

    let fut_resp: FutureResponse = client.get(uri);

    Box::new(fut_resp.and_then(|resp| {
        let mut result: String;

        let con: futures::stream::Concat2<hyper::Body> = resp.body().concat2();

        let result = con.map(move |full_body: hyper::Chunk| {
            let ret_str: String = match str::from_utf8(&full_body) {
                Ok(v) => v.to_string(),
                //todo remain calm and don't panic!
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            ret_str
        });

        result
    }))
}
