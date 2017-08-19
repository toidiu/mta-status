//#![deny(warnings)]
//#![allow(unused)]
//#![feature(conservative_impl_trait)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;
extern crate futures;

use futures::Future;
use tokio_core::reactor::Handle;

mod mta_client;
mod file_cache;
mod parse;

#[allow(dead_code)]
//todo enable caching to limit the number of requests to the MTA api
fn init() {
    file_cache::create_cache_file();
}

pub fn get_status(handle: &Handle) -> Box<Future<Item = String, Error = hyper::Error>> {
    // A good demonstration of a long running operation.
    // What do you expect this will do to concurrent requests?
    // use std::thread;
    // use std::time::Duration;
    // thread::sleep(Duration::from_secs(2));

    let fut_xml_resp = mta_client::request_status(handle);

    let fut_json_resp = fut_xml_resp.map(|xml_resp| {
        // Only pass a reference so that we can reuse the xml_resp for other
        // purposes such as logging.
        let json_str = parse::parse_xml::parse(&xml_resp);
        match serde_json::to_string(&json_str) {
            Ok(json) => json,
            Err(_) => "error".to_string(),
        }
    });

    Box::new(fut_json_resp)
}


#[cfg(test)]
mod tests {
    use super::mta_client;

    #[test]
    fn it_fails() {
        ::mta_client::connect();
    }
}
