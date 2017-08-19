extern crate hyper;
extern crate futures;
extern crate tokio_core;

use std::str;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Handle;

/// The client returns a `String` and therefore ownership of the result because
/// it is not used by the client.
///
/// Transfering ownership grarantees that the response wont be altered at a later
/// time by this function!
pub fn request_status(handle: &Handle) -> Box<Future<Item = String, Error = hyper::Error>> {
    // This is not a txt file, but actually a URL which returns a XML response with HTML
    // embedded inside.. yuk. The purpose of this project is to take that XML/HTML
    // response and convert it into a nice JSON response :)
    let uri: hyper::Uri = "http://web.mta.info/status/serviceStatus.txt"
        .parse()
        .unwrap();

    let fut_resp = Client::new(handle).get(uri)
        //todo check if this succeeds with `then`
        .and_then(|resp| {
            resp.body().concat2().map(move |chunk_body: hyper::Chunk| {
                match str::from_utf8(&chunk_body) {
                    Ok(v) => v.to_string(),
                    Err(_) => "{}".to_string(),
                }
            })
        });
    Box::new(fut_resp)
    //Box::new(futures::future::ok("doing".to_string()))
}
