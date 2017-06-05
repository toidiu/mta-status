//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate quick_xml;
extern crate mta_status;

//use std::io::Read;
use quick_xml::reader::Reader;
use quick_xml::events::Event;
//use hyper::Client;
//use hyper::server::{Server, Request, Response};
//use hyper::uri::RequestUri::AbsolutePath;
use mta_status::xml_client;
use mta_status::parse_xml;

fn main() {

    xml_client::connect();
    //    fn hello(req: Request, res: Response) {
    //        //        let e = AbsolutePath("/hi".to_string());
    //        //        let q = AbsolutePath("/yo".to_string());
    ////        match (req.method, req.uri) {
    ////            //            //(&Get, "/") | (&Get, "/echo") => {
    ////            //            (Get, AbsolutePath("/hi".to_string()))  => {
    ////            //                res.send(b"Hello World!").unwrap();
    ////            //            },
    ////            //            (Get, q)  => {
    ////            //                res.send(b"Hello dd!").unwrap();
    ////            //            }
    ////        }
    //        res.send(b"Hello World!").unwrap();
    //    }
    //
    //    Server::http("localhost:4000").unwrap().handle(hello).unwrap();

    //    pretty_env_logger::init().unwrap();
    println!("=================================");
    let mut xml_resp = String::new();
    xml_client::get_mta_status(&mut xml_resp);
    parse_xml::parse_xml(&xml_resp);

}
