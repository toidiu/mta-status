//#![deny(warnings)]
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde_json;

use hyper::Client;

pub mod xml_client;
pub mod parse_xml;


pub fn get_status() -> String {
    let client = Client::new();
    let result_xml_resp = xml_client::get_mta_status(&client);
    match result_xml_resp {
        Ok(mut xml_resp) => {
            let query = parse_xml::parse_xml(&mut xml_resp);
                match serde_json::to_string(&query) {
                    Ok(query) => query,
                    Err(_) => "error".to_string(),
                }
        },
        Err(_) => panic!("d") //res.send("error with request".as_bytes()).unwrap(),
    }


}

#[cfg(test)]
mod tests {

    use super::xml_client;

    #[test]
    fn it_fails() {
        ::xml_client::connect();
    }
}
