extern crate hyper;

use self::hyper::Client;
use std::io::Read;

pub fn connect() {
    println!("=============client=============");
}

pub fn get_mta_status(xml: &mut String) {
    let client = Client::new();
    client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send()
        .unwrap()
        .read_to_string(xml)
        .unwrap();
}
