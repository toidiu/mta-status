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
    let uri: hyper::Uri = "http://web.mta.info/status/serviceStatus.txt".parse().unwrap();

    let client = Client::new(&handle);
    //    let client = hyper::Client::configure()
    //        .keep_alive(true)
    //        .build(&handle);

    let fr: FutureResponse = client.get(uri);

    Box::new(fr.and_then(|r| {
        let mut result: String = r.status().to_string();

        let body: hyper::Body = r.body();
        let con: futures::stream::Concat2<hyper::Body> = body.concat2();


        let result = con.map(move |full_body: hyper::Chunk| {
            //                match full_body {
            //                    Ok(valid_body) => {
            //                        let d: String = valid_body;
            //                        d
            //                        ////                    Box::new(futures::future::ok("doing".to_string()).map(|qw| { "bla".to_string()}))
            //                        ////                    futures::future::ok("doing".to_string()).map(|qw| { "bla".to_string()})
            //                        //                    futures::future::ok("doing".to_string())
            let res: Vec<u8> = full_body.to_vec();
            str::from_utf8(&res).unwrap().to_string()
//            match {
//                Ok(v) => v,
//                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//            }
            //                res
            //        "cheese".to_string()
            //                    },
            //                    Err(e) =>  "".to_string(),
        });


        result
    }))
    //        Box::new(futures::future::ok("doing".to_string())
}
