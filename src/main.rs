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


    fn get_mta_status(_: Request, res: Response) {
        let client = Client::new();

        let mut xml_resp = xml_client::get_mta_status(&client);
        let query = parse_xml::parse_xml(&mut xml_resp);

        let result_query = serde_json::to_string(&query);

        match result_query {
            Ok(query) => res.send(query.as_bytes()),
            Err(e) => res.send("error with request".as_bytes()),
        }.unwrap()

    }

    println!("running at: http://localhost:4000");
    Server::http("localhost:4000").unwrap().handle(get_mta_status).unwrap();

}
