//#![deny(warnings)]
extern crate hyper;

use self::hyper::Client;
use std::io::Read;
use self::hyper::client::Response;

pub fn get_mta_status(client: &Client) -> String {
    let result_req = client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send();


    let mut xml_resp = String::new();

    match result_req {
        Ok(mut req) => {
            req.read_to_string(&mut xml_resp);
            ()
        }
        Err(e) => xml_resp = "ERROR".to_string(),
    };

    xml_resp
}
