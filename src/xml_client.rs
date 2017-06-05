extern crate hyper;

use self::hyper::Client;
use std::io::Read;

pub fn get_mta_status(client: &Client, xml: &mut String) {
    client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send()
        .unwrap()
        .read_to_string(xml)
        .unwrap();
}
