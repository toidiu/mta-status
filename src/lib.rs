//#![deny(warnings)]
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde_json;

use hyper::Client;
//use std::thread;
//use std::time::Duration;

mod xml_client;
mod parse_xml;
mod file_cache;

pub fn init() {
    file_cache::create_cache_file();
}

pub fn get_status() -> String {

    //thread::sleep(Duration::from_secs(5));
    let client = Client::new();
    let result_xml_resp = xml_client::get_mta_status(&client);
    let status = match result_xml_resp {
        Ok(mut xml_resp) => {
            let query = parse_xml::parse_xml(&mut xml_resp);
                match serde_json::to_string(&query) {
                    Ok(query) => query,
                    Err(_) => "error".to_string(),
                }
        },
        Err(_) => panic!("Unable to get status form http://web.mta.info")
    };

    status
}


#[cfg(test)]
mod tests {

    use super::xml_client;

    #[test]
    fn it_fails() {
        ::xml_client::connect();
    }
}
