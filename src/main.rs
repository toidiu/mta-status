//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate mta_status;
extern crate serde_json;

extern crate quick_xml;
use self::quick_xml::reader::Reader;
use self::quick_xml::events::Event;




use hyper::Client;
use hyper::server::{Server, Request, Response};
//use hyper::uri::RequestUri::AbsolutePath;
use mta_status::xml_client;
use mta_status::parse_xml;

fn main() {

    fn get_mta_status(req: Request, res: Response) {

        let client = Client::new();
        let mut xml_resp = String::new();

        xml_client::get_mta_status(&client, &mut xml_resp);
        let query = parse_xml::parse_xml(&mut xml_resp);
//        parse_xml(&mut xml_resp);

//    let query = format!("{:?}", query);
    let query = serde_json::to_string(&query).unwrap();
//    println!("{:?}", query);
//        res.send(b"Hello Worldasdf!").unwrap();
        res.send(query.as_bytes()).unwrap();
//    println!("{:?}", query);
    }


    Server::http("localhost:4000").unwrap().handle(get_mta_status).unwrap();

    println!("=================================");
}
