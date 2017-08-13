#![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;


pub static CACHE_FILE: &'static str = "cache.txt";

pub fn create_cache_file() {
    let try_file = File::open(CACHE_FILE);
    match try_file {
        Ok(file) => file,
        Err(_) => File::create(CACHE_FILE).unwrap(),
    };
}

pub fn read_status_file() -> String {
    let mut file = File::open(CACHE_FILE).expect("unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read from file");
    contents
}

pub fn write_status_file(content: &str) {

    let mut file = OpenOptions::new().write(true).open(CACHE_FILE).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
