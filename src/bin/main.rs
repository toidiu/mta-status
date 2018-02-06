//#![deny(warnings)]
//#![allow(unused, deprecated)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
// #![feature(use_extern_macros)]

#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate futures;
extern crate log4rs;
extern crate mta_status;
extern crate net2;
extern crate num_cpus;
extern crate tokio_core;
extern crate pretty_env_logger;

use std::fs::File;
use std::io::Write;


mod service;


#[cfg(debug_assertions)]
const IS_PROD: bool = false;

#[cfg(not(debug_assertions))]
const IS_PROD: bool = true;



fn main() {

    if IS_PROD {
        let data = include_str!("../../resources/log4rs.yaml");
        let mut f = File::create("log_config.yaml").expect("Unable to create file");
        f.write_all(data.as_bytes()).expect("Unable to write data");
        log4rs::init_file("log_config.yaml", Default::default()).unwrap();
    } else {
        pretty_env_logger::init();
    }

    let url = "127.0.0.1:4000";
    warn!("prod build: {}", IS_PROD);
    warn!("http://{}", url);
    service::start_server(url, num_cpus::get());
}
