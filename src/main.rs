//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate mta_status;
extern crate serde_json;

use hyper::Client;
use hyper::server::{Server, Request, Response};
use mta_status::xml_client;
use mta_status::parse_xml;


fn main() {


//    fn get_mta_status(_: Request, res: Response) {
        let client = Client::new();
        let result_xml_resp = xml_client::get_mta_status(&client);

        match result_xml_resp {
            Ok(mut xml_resp) => {
                let query = parse_xml::parse_xml(&mut xml_resp);
//                match serde_json::to_string(&query) {
//                    Ok(query) => res.send(query.as_bytes()),
//                    Err(_) => res.send("error parsing json".as_bytes()),
//                }.unwrap()
            },
            Err(_) => panic!("d") //res.send("error with request".as_bytes()).unwrap(),
        };

//    }

//    println!("running at: http://localhost:4000");
//    Server::http("localhost:4000").unwrap().handle(get_mta_status).unwrap();

}
