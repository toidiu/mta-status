//#![deny(warnings)]
//#![allow(unused)]
//#![feature(conservative_impl_trait)]
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;
extern crate futures;

use futures::Future;
use tokio_core::reactor::Handle;

mod mta_client;
mod parse_xml;
mod file_cache;
mod parse_html;

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

    let result_xml_resp = mta_client::get_mta_status(handle);

    let result_xml_resp = result_xml_resp.map(|mut xml_resp| {
        let query = parse_xml::parse(&mut xml_resp);
        match serde_json::to_string(&query) {
            Ok(query) => query,
            Err(_) => "error".to_string(),
        }
    });

    Box::new(result_xml_resp)
}


#[cfg(test)]
mod tests {
    use super::mta_client;

    #[test]
    fn it_fails() {
        ::mta_client::connect();
    }
}
