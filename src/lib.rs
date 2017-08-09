//#![deny(warnings)]
#![feature(conservative_impl_trait)]
#[macro_use]

extern crate serde_derive;
extern crate hyper;
extern crate serde_json;
extern crate tokio_core;
extern crate futures;

use hyper::Client;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;

mod xml_client;
mod parse_xml;
mod file_cache;
pub mod helper_types;

pub fn init() {
    file_cache::create_cache_file();
}


pub fn fut() -> impl Future<Item = String, Error = hyper::Error> {
    futures::future::ok("hi".to_string())
}


pub fn get_status() -> impl Future<Item = String, Error = hyper::Error> {

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    //thread::sleep(Duration::from_secs(5));

    let result_xml_resp = xml_client::get_mta_status(handle.clone());
   // let status = match result_xml_resp {
   //     Ok(mut xml_resp) => {
   //         let query = parse_xml::parse_xml(&mut xml_resp);
   //             match serde_json::to_string(&query) {
   //                 Ok(query) => query,
   //                 Err(_) => "error".to_string(),
   //             }
   //     },
   //     Err(_) => panic!("Unable to get status form http://web.mta.info")
   // };


    //futures::future::ok("todo".to_string())
        result_xml_resp
}


#[cfg(test)]
mod tests {

    use super::xml_client;

    #[test]
    fn it_fails() {
        ::xml_client::connect();
    }
}
