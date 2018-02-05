//#![deny(warnings)]
//#![allow(unused, deprecated)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
// #![feature(use_extern_macros)]

#[macro_use] extern crate hyper;
#[macro_use] extern crate log;
extern crate futures;
extern crate log4rs;
extern crate mta_status;
extern crate net2;
extern crate num_cpus;
extern crate tokio_core;

use std::fs::File;
use std::io::Write;


mod service;


#[cfg(debug_assertions)]
const IS_PROD: bool = false;

#[cfg(not(debug_assertions))]
const IS_PROD: bool = true;



fn main() {

    let data = include_str!("../../resources/log4rs.yaml");
    let mut f = File::create("log_config.yaml").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    info!("prod build: {}", IS_PROD);
    info!("http://localhost:4000");

    service::start_better_server("127.0.0.1:4000", num_cpus::get());
}
