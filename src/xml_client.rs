//#![deny(warnings)]
extern crate hyper;
extern crate futures;
extern crate tokio_core;

//use self::hyper::Client;
use std::io::Read;
use hyper::client::HttpConnector;
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::client::Response;
use futures::future::BoxFuture;
use hyper::client::FutureResponse;
use futures::future::Map;
pub fn get_mta_status(handle: tokio_core::reactor::Handle) -> BoxFuture<String, hyper::Error> {
    let uri: hyper::Uri = "http://web.mta.info/status/serviceStatus.txt".parse().unwrap();


    let client = Client::new(&handle);
    let fr: FutureResponse =
        client.get(uri);
//
//    let map: Map<FutureResponse, _>  = fr.map(|resp| {
//          //  println!("adf {}", dd.status());
//          resp.status().to_string()
////        "asdf".to_string()
//            //futures::future::ok("doing".to_string()).map(|qw| { "bla".to_string()}).boxed()
//            //
//        });
//    map.boxed()



    futures::future::ok("doing".to_string()).map(|qw| { "bla".to_string()}).boxed()
//        let result_req = client
//        .get("http://web.mta.info/status/serviceStatus.txt")
//        .send();
//
//    let mut xml_resp = String::new();
//    match result_req {
//        Ok(mut req) => {
//            let _ = req.read_to_string(&mut xml_resp);
//            Ok(xml_resp)
//        }
//        Err(_) => Err("ERROR".to_string()),
//    }



 //   let s = result_req.map(|resp: Response| {

 //           match resp {
 //               Ok(mut req) => {
 //                   let mut xml_resp = String::new();
 //                   let _ = req.read_to_string(&mut xml_resp);
 //                   Ok(xml_resp)
 //               }
 //               Err(_) => Err("Error getting mta response".to_string()),
 //           }


 //   });

 //   s

}
