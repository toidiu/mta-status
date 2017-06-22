//#![deny(warnings)]
extern crate hyper;

use self::hyper::Client;
use std::io::Read;

pub fn get_mta_status(client: &Client, xml: &mut String) {
    let result_req = client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send();


//    result_req.unwrap().read_to_string(xml);
        match result_req {
            Ok(mut req) => {
                req.read_to_string(xml);
                    ()
            }  ,
            Err(e) => (),
        };
}
