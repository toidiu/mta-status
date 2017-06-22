#![deny(warnings)]
extern crate hyper;

use self::hyper::Client;
use std::io::Read;

pub fn get_mta_status(client: &Client) -> Result<String, String> {
    let result_req = client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send();

    let mut xml_resp = String::new();
    match result_req {
        Ok(mut req) => {
            let _ = req.read_to_string(&mut xml_resp);
            Ok(xml_resp)
        }
        Err(_) => Err("ERROR".to_string()),
    }

}
