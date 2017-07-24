//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate mta_status;
extern crate serde_json;

use hyper::server::{Server, Request, Response};


fn main() {


   fn get_mta_status(_: Request, res: Response) {
        let mta_status = mta_status::get_status();

        res.send(mta_status.as_bytes());
    }


    mta_status::init();
    println!("running at: http://localhost:4000");
    Server::http("localhost:4000").unwrap().handle(get_mta_status).unwrap();

}
